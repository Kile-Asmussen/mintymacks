use std::collections::HashMap;

use rand::{SeedableRng, rngs::SmallRng, seq::IndexedRandom};

use anyhow::anyhow;

use crate::{
    bits::{Bits, board::BitBoard, movegen::pawn_moves, show_mask},
    fuzzing::stockfish_perft,
    model::{Color, ColorPiece, Piece, Square, castling::CastlingRights, moves::PseudoMove},
    uci::{
        fen::{self, parse_fen_board, render_fen_board},
        perft,
    },
    zobrist::{self, Hash, ZobristBoard},
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
    let mut positions = HashMap::new();

    for _ in 0..10000 {
        zobrist_hashing_game(&mut rng, 100, &mut positions);
    }

    println!("Positions seen: {}", positions.len());
}

fn zobrist_hashing_game(rng: &mut SmallRng, ply: usize, positions: &mut HashMap<Hash, BitBoard>) {
    let zobrist = ZobristBoard::new();

    let mut buf = vec![];

    let mut board = BitBoard::startpos();

    for _ in 0..ply {
        let hash = zobrist.hash(&board);

        if let Some(b) = positions.get(&hash)
            && &board != b
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
    let zobrist = ZobristBoard::new();

    for _ in 0..100 {
        zobrist_delta_game(&mut rng, 50, &zobrist);
    }
}

fn zobrist_delta_game(rng: &mut SmallRng, ply: usize, zobrist: &ZobristBoard) {
    let mut buf = vec![];
    let mut moves = vec![];
    let mut board = BitBoard::startpos();
    let mut hash = zobrist.hash(&board);

    for _ in 0..ply {
        buf.clear();
        board.moves(&mut buf);

        if let Some(mv) = buf.choose(rng) {
            let mv = *mv;
            moves.push(mv);
            board.apply(mv);
            hash ^= zobrist.delta(mv, board.metadata.castling_details);
            let reference = zobrist.hash(&board);

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

fn stockfish_comparison_game(
    rng: &mut SmallRng,
    ply: usize,
    skip: usize,
    depth: usize,
    start: &[(PseudoMove, Option<Piece>)],
) -> anyhow::Result<()> {
    let mut buf = vec![];
    let mut board = BitBoard::startpos();
    let mut moves = board.make_moves(start);

    if moves.len() != start.len() {
        panic!("Bad starting moves")
    }

    let mut problems = vec![];

    'ply: for _ in 0..=ply {
        let stock = stockfish_perft(&moves, depth)?;
        let mut mint = board.perft(depth).moves;

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
                    "{} not found in mintymacks perft",
                    stock_move.0.longalg(stock_move.1)
                ));
            }
        }
        for (mint_move, mint_num) in mint {
            problems.push(format!(
                "{} not found in stockfish perft",
                mint_move.0.longalg(mint_move.1)
            ));
        }

        if problems.len() > 0 {
            break;
        }

        for _ in 0..=skip {
            buf.clear();
            board.moves(&mut buf);

            if let Some(mv) = buf.choose(rng) {
                moves.push(*mv);
                board.apply(*mv);
            } else {
                break 'ply;
            }
        }
    }

    if problems.len() > 0 {
        println!("Perft mismatch!");
        println!("FEN: {}", render_fen_board(&board.render()));
        println!(
            "Moves: [{}]",
            moves
                .into_iter()
                .map(|m| m.longalg())
                .collect::<Vec<_>>()
                .join(" ")
        );

        for p in problems {
            println!("  {}", p);
        }

        panic!();
    }

    println!(
        "Game of {} moves successfully played in accordance with stockfish",
        moves.len()
    );

    Ok(())
}

#[test]
fn fuzz_stockfish_comparison() {
    let mut rng = pi();

    stockfish_comparison_game(&mut rng, 50, 0, 1, &[]);
}
