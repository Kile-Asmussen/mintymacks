use std::{
    collections::HashMap,
    io::{Write, stdout},
    time::Instant,
};

use rand::{RngCore, SeedableRng, rngs::SmallRng, seq::IndexedRandom};

use anyhow::anyhow;

use crate::{
    arrays::ArrayBoard,
    bits::{
        Bits,
        board::BitBoard,
        movegen::{king_moves, pawn_moves},
        show_mask,
    },
    fuzzing::stockfish_perft,
    model::{
        ChessPiece, Color, ColoredChessPiece, Square,
        castling::{CLASSIC_CASTLING, CastlingRights},
        moves::PseudoMove,
    },
    notation::{
        fen::{self, parse_fen, parse_fen_board, render_fen_board},
        pgn::load_pgn_file,
    },
    zobrist::{self, ZobHash, ZobristBoard},
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
    let zobrist = ZobristBoard::new();

    let mut buf = vec![];

    let mut board = BitBoard::startpos();

    for _ in 0..ply {
        let hash = zobrist.hash(&board);

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

#[cfg(test)]
fn stockfish_comparison_game(
    rng: &mut SmallRng,
    ply: usize,
    skip_over: usize,
    depth: usize,
    start: &[(PseudoMove, Option<ChessPiece>)],
    skip_this: bool,
) {
    let mut buf = vec![];
    let mut board = BitBoard::startpos();
    let mut moves = board.apply_pseudomoves(start);

    if moves.len() != start.len() {
        panic!("Bad starting moves")
    }

    let now = Instant::now();
    let mut problems = vec![];

    println!("Playing {} ply: ", ply + 1);
    if skip_this {
        println!("(skipping stockfish eval)");
    }

    'ply: loop {
        let mut mint = board.enumerate(depth).moves;
        let stock = if skip_this {
            mint.clone()
        } else {
            stockfish_perft(&moves, depth).expect("Unable to get stockfish perft!")
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
                if moves.len() % 8 == 0 {
                    if moves.len() != 0 {
                        println!();
                    }
                    print!("{:03}> ", moves.len());
                }
                moves.push(*mv);
                board.apply(*mv);
                print!("{} ", mv.longalg());
                stdout().flush();
            } else {
                break 'ply;
            }
        }

        if moves.len() > ply + skip_over {
            break 'ply;
        }
    }

    if problems.len() > 0 {
        use crate::notation::fen::render_fen;

        println!();
        println!("Perft mismatch!");
        println!("FEN: {}", render_fen(&board, 0));

        for p in problems {
            println!("- {}", p);
        }

        panic!();
    } else {
        println!();
        println!(
            "Game of {} ply successfully played in accordance with stockfish in {} seconds",
            moves.len(),
            now.elapsed().as_secs(),
        );
    }
}

#[cfg(test)]
pub fn fuzz_stockfish_comparison(
    rng: &mut SmallRng,
    n: usize,
    skip_to: usize,
    ply: usize,
    depth: usize,
    step: usize,
) {
    println!("Playing {n} random games and comparing to stockfish...");

    for i in 1..=n {
        println!("\n### Game {i} ###");
        stockfish_comparison_game(rng, ply - 1, step - 1, depth, &[], i < skip_to);
    }

    println!();
    println!("!!! Successfully played {n} random games in accordance with stockfish !!!");
}

#[test]
fn fuzz_against_stockfish() {
    let mut rng = pi();
    rng.next_u64();
    fuzz_stockfish_comparison(&mut rng, 10, 0, 48, 3, 1);
}

const PROBLEM: &str = r#"
[White "Stockfish"]
[Black "Stockfish"]

1. e4 c5
2. Nf3 d6
3. d4 cxd4
4. Nxd4 Nf6
5. Nc3 Nc6
6. Bg5 Bd7
7. Be2 a6
8. Bxf6+ gxf6
9. Nf5 Qb8
10. Nd5 Qd8
11. Ng3 e6
12. Ne3 Qb6
13. O-O O-O-O
14. c3 Ne5
15. Qd4 Qc5
16. Qd2 Qc7
17. b4 h5
18. Nxh5 Bc6
19. f4 Ng6
20. Nxf6 Be7
21. Neg4 Bxf6
22. Nxf6 Qe7
23. Qd4 Nxf4
24. Bxa6 Kb8
25. Rxf4 e5
26. b5 exd4+
27. bxc6 bxc6
28. cxd4 Ka8
29. Rd1 Rb8
30. Be2 Qa7
31. Bf3 Qa3
32. Kh1 Qe3
33. Rf5 Qc3
34. Ng4 Rb4
35. h3 Rxd4
36. Rf1 Rb8
37. e5 d5
38. Rxf7 Rdb4
39. Kh2 Rg8
40. Be2 Re4
41. R1f3 Qc1
42. Bf1 Rh8
43. Nf2 Re3
44. R3f4 Qa3
45. R4f6 Rc8
46. Ng4 Re1
47. Rf2 Qa5
48. g3 Rc7
49. R7f4 Ka7
50. Bg2 Qb5
51. Bf1 Qa5
52. Bg2 Qa6
53. Bf1 Qa3
54. Bg2 Rb7
55. Rc2 Rb2
56. Rff2 Rxc2
57. Rxc2 Qd3
58. Rf2 Qd4
59. Bf1 c5
60. Rf4 Qd2+
61. Rf2 Qd1
62. Rf7+ Kb6
63. Rf6+ Kc7
64. Bg2 Qd4
65. Rf1 Rxf1
66. Bxf1 c4
67. Bg2 c3
68. Nf6 c2
69. Nxd5+ Kd7
70. Nf4 Qd2
71. e6+ Kd6
72. h4 c1=Q
73. Kh3 Qf2
74. e7 Kxe7
75. Nd5+ Kd6
76. Nf4 *
    "#;

#[test]
fn cornercase() {
    let mut pgn = load_pgn_file(PROBLEM);

    let pgn = pgn.pop().unwrap();

    let mut board1 = BitBoard::startpos();
    let amvs = pgn.move_list();
    let cmvs = board1.apply_algebraics(&amvs);

    assert_eq!(amvs.len(), cmvs.len());

    let mut board2 = parse_fen("8/8/3k4/8/5N1P/6PK/P4qB1/2q5 b - - 0 76")
        .unwrap()
        .0;
    assert_eq!(board1.white, board2.white, "WHITE");
    assert_eq!(board1.black, board2.black, "BLACK");
}
