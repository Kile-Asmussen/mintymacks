use std::{
    collections::{HashMap, VecDeque},
    io::Write,
    path::Path,
    time::{Duration, Instant},
};

use rand::{RngCore, SeedableRng, rngs::SmallRng, seq::IndexedRandom};
use tokio::io::{AsyncWriteExt, stdout};

use anyhow::anyhow;

#[cfg(test)]
use crate::engine::EngineHandle;
use crate::{
    arrays::ArrayBoard,
    bits::{
        Bits,
        board::BitBoard,
        movegen::{king_moves, pawn_moves},
        show_mask,
    },
    deque,
    fuzzing::stockfish_perft,
    model::{
        ChessPiece, Color, ColoredChessPiece, Square,
        castling::{CLASSIC_CASTLING, CastlingRights},
        moves::PseudoMove,
    },
    notation::{
        fen::{self, parse_fen, parse_fen_board, render_fen_board},
        pgn::load_pgn_file,
        uci::gui::UciGui,
    },
    zobrist::{self, ZOBHASHER, ZobHash, ZobristBoard},
};

fn pi() -> SmallRng {
    rand::prelude::SmallRng::from_seed(*b"3.141592653589793238462643383279")
}

#[test]
fn fuzz_unmaking_moves() {
    let mut rng = pi();

    for _ in 0..1000 {
        unmake_moves(&mut rng, 50);
    }
}

fn unmake_moves(rng: &mut SmallRng, ply: usize) {
    let mut moves = vec![];

    let mut buf = vec![];

    let mut board = BitBoard::startpos();

    for _ in 0..ply {
        buf.clear();
        board.moves(&mut buf);

        if let Some(mv) = buf.choose(rng) {
            let mv = *mv;

            moves.push((board.clone(), mv));
            board.apply(mv);
        } else {
            break;
        }
    }

    let all_moves = moves.iter().map(|(_, m)| *m).collect::<Vec<_>>();
    let mut unmade_moves = vec![];

    while let Some((b, mv)) = moves.pop() {
        board.unapply(mv);
        unmade_moves.push(mv);

        if board != b {
            println!("Unmade move mismatch!");
            println!(
                "All moves: {}",
                all_moves
                    .iter()
                    .map(|m| m.longalg())
                    .collect::<Vec<_>>()
                    .join(" ")
            );
            unmade_moves.reverse();
            println!(
                "Unmade moves: {}",
                unmade_moves
                    .iter()
                    .map(|m| m.longalg())
                    .collect::<Vec<_>>()
                    .join(" ")
            );
            println!("Position: {}", render_fen_board(&board.render()));
            println!("Metadata: {:?}", board.metadata);
            println!("Expected position: {}", render_fen_board(&b.render()));
            println!("Expected metadata: {:?}", b.metadata);
            panic!();
        }
    }
}

#[test]
fn fuzz_zobrist_hashing() {
    let mut rng = pi();
    let mut positions = hash_map! {};

    for _ in 0..10000 {
        zobrist_hashing_game(&mut rng, 100, &mut positions);
    }

    println!("Positions seen: {}", positions.len());
}

fn zobrist_hashing_game(
    rng: &mut SmallRng,
    ply: usize,
    positions: &mut HashMap<ZobHash, BitBoard>,
) {
    let mut buf = vec![];

    let mut board = BitBoard::startpos();

    for _ in 0..ply {
        let hash = ZOBHASHER.hash(&board);

        if let Some(b) = positions.get(&hash)
            && board.white != b.white
            && board.black != b.black
            && board.metadata.equiv(&b.metadata)
        {
            println!("Hash Colission found: {}", hash);
            println!("Current position: {}", render_fen_board(&board.render()));
            println!("Current metadata: {:?}", board.metadata);
            println!(
                "Previously seen position: {}",
                render_fen_board(&b.render())
            );
            println!("Previously seen metadata: {:?}", b.metadata);
            panic!();
        }

        positions.insert(hash, board.clone());

        buf.clear();
        board.moves(&mut buf);

        if let Some(mv) = buf.choose(rng) {
            let mv = *mv;

            board.apply(mv);
        } else {
            break;
        }
    }
}

#[test]
fn fuzz_zobrist_delta() {
    let mut rng = pi();

    for _ in 0..100 {
        zobrist_delta_game(&mut rng, 50);
    }
}

fn zobrist_delta_game(rng: &mut SmallRng, ply: usize) {
    let mut buf = vec![];
    let mut moves = vec![];
    let mut board = BitBoard::startpos();
    let mut hash = ZOBHASHER.hash(&board);

    for _ in 0..ply {
        buf.clear();
        board.moves(&mut buf);

        if let Some(mv) = buf.choose(rng) {
            let mv = *mv;
            moves.push(mv);
            board.apply(mv);
            hash ^= ZOBHASHER.delta(mv, board.metadata.castling_details);
            let reference = ZOBHASHER.hash(&board);

            if hash != reference {
                println!("Hash mismatch! {:X} != {:X}", hash, reference);
                println!("Board state {}", render_fen_board(&board.render()));
                println!("Board metadata {:?}", board.metadata);
                println!(
                    "Move sequence {}",
                    moves
                        .iter()
                        .map(|m| m.longalg())
                        .collect::<Vec<_>>()
                        .join(" ")
                );
                panic!();
            }
        } else {
            break;
        }
    }
}

#[cfg(test)]
async fn stockfish_comparison_game(
    engine: &mut EngineHandle,
    rng: &mut SmallRng,
    ply: usize,
    skip_over: usize,
    depth: usize,
    start: &[(PseudoMove, Option<ChessPiece>)],
    skip_this: bool,
) {
    use crate::println_async;

    let mut buf = vec![];
    let mut board = BitBoard::startpos();
    let mut moves = board.apply_pseudomoves(start);

    if moves.len() != start.len() {
        panic!("Bad starting moves")
    }

    engine.interleave(
        &mut deque![UciGui::UciNewGame()],
        &mut vec![],
        Duration::from_millis(50),
    );

    let now = Instant::now();
    let mut problems = vec![];

    println_async!("Playing {} ply: ", ply + 1).await;
    if skip_this {
        use crate::println_async;

        println_async!("(skipping stockfish eval)").await;
    }

    'ply: loop {
        let mut mint = board.enumerate(depth).moves;
        let stock = if skip_this {
            mint.clone()
        } else {
            stockfish_perft(engine, &moves, depth)
                .await
                .expect("Unable to get stockfish perft!")
        };

        for (stock_move, stock_num) in stock {
            if let Some((mint_move, mint_num)) = mint.remove_entry(&stock_move) {
                if mint_num != stock_num {
                    problems.push(format!(
                        "{} has {} stockfish moves and {} mintymacks moves",
                        stock_move.0.longalg(stock_move.1),
                        stock_num,
                        mint_num
                    ));
                }
            } else {
                problems.push(format!(
                    "{:?} {} not found in mintymacks perft",
                    board.metadata.to_move,
                    stock_move.0.longalg(stock_move.1)
                ));
            }
        }
        for (mint_move, mint_num) in mint {
            problems.push(format!(
                "{:?} {} not found in stockfish perft",
                board.metadata.to_move,
                mint_move.0.longalg(mint_move.1)
            ));
        }

        if problems.len() > 0 {
            break;
        }

        for _ in 0..=skip_over {
            buf.clear();
            board.moves(&mut buf);

            if let Some(mv) = buf.choose(rng) {
                use crate::print_async;

                if moves.len() % 8 == 0 {
                    if moves.len() != 0 {
                        use crate::println_async;
                        println_async!().await;
                    }
                    print_async!("{:03}> ", moves.len()).await;
                }
                moves.push(*mv);
                board.apply(*mv);
                print_async!("{} ", mv.longalg()).await;
            } else {
                break 'ply;
            }
        }

        if moves.len() > ply + skip_over {
            break 'ply;
        }
    }

    if problems.len() > 0 {
        use crate::{notation::fen::render_fen, println_async};

        println_async!().await;
        println_async!("Perft mismatch!").await;
        println_async!("FEN: {}", render_fen(&board, 0)).await;

        for p in problems {
            println_async!("- {}", p).await;
        }

        panic!();
    } else {
        use crate::println_async;

        println_async!().await;
        println_async!(
            "Game of {} ply successfully played in accordance with stockfish in {} seconds",
            moves.len(),
            now.elapsed().as_secs(),
        )
        .await;
    }
}

#[cfg(test)]
pub async fn fuzz_stockfish_comparison(
    engine: &mut EngineHandle,
    rng: &mut SmallRng,
    n: usize,
    skip_to: usize,
    ply: usize,
    depth: usize,
    step: usize,
) {
    use crate::println_async;

    println_async!("Playing {n} random games and comparing to stockfish...\n").await;

    for i in 1..=n {
        use crate::println_async;

        println_async!("\n### Game {i} ###").await;
        stockfish_comparison_game(engine, rng, ply - 1, step - 1, depth, &[], i < skip_to).await;
    }

    println_async!("\n !!! Successfully played {n} random games in accordance with stockfish !!!")
        .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn fuzz_against_stockfish() {
    let mut rng = pi();
    let mut engine = EngineHandle::open(Path::new("stockfish"), &[] as &[&str], false)
        .await
        .expect("Could not open stockfish");

    engine
        .interleave(
            &mut deque![UciGui::Uci()],
            &mut vec![],
            Duration::from_millis(500),
        )
        .await;

    rng.next_u64();
    fuzz_stockfish_comparison(&mut engine, &mut rng, 10_000, 0, 96, 3, 1).await;
}
