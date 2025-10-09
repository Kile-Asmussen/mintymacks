use crate::{
    arrays::ArrayBoard,
    bits::{
        Bits, BoardMask, bit,
        board::{BitBoard, HalfBitBoard},
        jumps::KNIGHT_MOVES,
        mask,
        movegen::{legal_moves, pawn_moves},
        show_mask, slides,
        threats::{bishop_threats, knight_threats, rook_threats},
    },
    fuzzing::stockfish_perft,
    model::{
        Color, ColoredChessPiece, Square,
        castling::{CLASSIC_CASTLING, CastlingRights},
        metadata::Metadata,
        moves::{ChessMove, PseudoMove},
    },
    notation::{
        algebraic,
        fen::{self, parse_fen, parse_fen_board, render_fen},
    },
};

#[test]
fn knight_threat_masks() {
    let t = knight_threats(mask([
        0b_00000000,
        0b_00000000,
        0b_00000100,
        0b_00000000,
        0b_00000000,
        0b_00100000,
        0b_00000000,
        0b_00000000,
    ]));

    assert_eq!(
        t,
        mask([
            0b_00001010,
            0b_00010001,
            0b_00000000,
            0b_01010001,
            0b_10001010,
            0b_00000000,
            0b_10001000,
            0b_01010000,
        ])
    );
}

#[test]
fn rook_threat_masks() {
    let t = rook_threats(
        mask([
            0b_00000000,
            0b_00000000,
            0b_00000100,
            0b_00000000,
            0b_00000000,
            0b_00100000,
            0b_00000000,
            0b_00000000,
        ]),
        mask([
            0b_00000000,
            0b_00000100,
            0b_00000110,
            0b_00000000,
            0b_00000000,
            0b_00101000,
            0b_00000000,
            0b_00000000,
        ]),
    );

    assert_eq!(
        t,
        mask([
            0b_00100000,
            0b_00100100,
            0b_11111010,
            0b_00100100,
            0b_00100100,
            0b_11011100,
            0b_00100100,
            0b_00100100,
        ])
    )
}

#[test]
fn bishop_threat_masks() {
    let t = bishop_threats(
        mask([
            0b_00000000,
            0b_00000000,
            0b_00000100,
            0b_00000000,
            0b_00000000,
            0b_00100000,
            0b_00000000,
            0b_00000000,
        ]),
        mask([
            0b_00000000,
            0b_00000000,
            0b_00000100,
            0b_00000000,
            0b_00000000,
            0b_00100000,
            0b_00000000,
            0b_00000000,
        ]),
    );

    assert_eq!(
        t,
        mask([
            0b_00010001,
            0b_00001010,
            0b_00000100,
            0b_10001010,
            0b_01010001,
            0b_00100000,
            0b_01010000,
            0b_10001000,
        ])
    )
}

fn test_move_numbers(fen: &str, c: Color, cr: CastlingRights, epc: Option<Square>, num: usize) {
    let board = fen::parse_fen_board(fen).unwrap();

    let white = HalfBitBoard::new(Color::White, &board);
    let black = HalfBitBoard::new(Color::Black, &board);
    let metadata = Metadata {
        to_move: c,
        castling_rights: cr,
        turn: 1,
        en_passant: epc,
        castling_details: CLASSIC_CASTLING,
    };

    let mut moves = vec![];
    legal_moves(&white, &black, metadata, &mut moves);
    println!("FEN: {}", fen);
    println!(
        "moves: {}",
        moves
            .iter()
            .map(|x| x.longalg())
            .collect::<Vec<_>>()
            .join(" ")
    );
    assert_eq!(moves.len(), num);
}

#[test]
fn test_movegen() {
    test_move_numbers(
        "8/8/8/8/8/8/8/8",
        Color::White,
        CastlingRights::nil(),
        None,
        0,
    );

    test_move_numbers(
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR",
        Color::White,
        CastlingRights::full(),
        None,
        20,
    );

    test_move_numbers(
        "R7/8/8/8/8/8/8/8",
        Color::White,
        CastlingRights::nil(),
        None,
        14,
    );

    test_move_numbers(
        "R6R/3Q4/1Q4Q1/4Q3/2Q4Q/Q4Q2/pp1Q4/kBNN1KB1",
        Color::White,
        CastlingRights::nil(),
        None,
        218,
    );
}

#[test]
fn test_moving() {
    let mut board = BitBoard::startpos();
    board.apply(ChessMove {
        piece: ColoredChessPiece::WhitePawn,
        pmv: Square::d2.to(Square::d4),
        cap: None,
        special: None,
        rights: CastlingRights::full(),
        epc: None,
    });

    assert_eq!(
        board.render(),
        fen::parse_fen_board("rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR").unwrap()
    );
}

#[test]
fn test_knight_move_corner_case() {
    assert_eq!(
        KNIGHT_MOVES.at(Square::c1),
        mask([
            0b_00000000,
            0b_00000000,
            0b_00000000,
            0b_00000000,
            0b_00000000,
            0b_01010000,
            0b_10001000,
            0b_00000000,
        ])
    );

    assert_eq!(
        KNIGHT_MOVES.at(Square::d1),
        mask([
            0b_00000000,
            0b_00000000,
            0b_00000000,
            0b_00000000,
            0b_00000000,
            0b_00101000,
            0b_01000100,
            0b_00000000,
        ])
    );

    assert_eq!(
        KNIGHT_MOVES.at(Square::g1),
        mask([
            0b_00000000,
            0b_00000000,
            0b_00000000,
            0b_00000000,
            0b_00000000,
            0b_00000101,
            0b_00001000,
            0b_00000000,
        ])
    );
}

#[test]
fn en_passant_pawn_capture() {
    let mut board = ArrayBoard::<Option<ColoredChessPiece>>::new(None);

    board.set(Square::a7, Some(ColoredChessPiece::BlackPawn));
    board.set(Square::b5, Some(ColoredChessPiece::WhitePawn));
    board.set(Square::h1, Some(ColoredChessPiece::WhiteKing));
    board.set(Square::h8, Some(ColoredChessPiece::BlackKing));

    let mut board = BitBoard::new(
        &board,
        Color::Black,
        1,
        CastlingRights::nil(),
        None,
        CLASSIC_CASTLING,
    );

    let mv = board
        .apply_pseudomove(Square::a7.to(Square::a5).p())
        .unwrap();

    println!("({:?}).epc_opening() == {:?}", mv, mv.ep_opening());
    println!();

    let mut res = vec![];
    board.moves(&mut res);

    for mv in &res {
        println!("{:?}", mv)
    }
}

#[test]
fn solve_this_debackle() {
    let fen = "r3kbnr/pp1b1ppp/1qn1p3/3pP3/3p4/2PB1N2/PP3PPP/RNBQ1RK1 w kq - 0 8";
    let (board, _) = parse_fen(fen).unwrap();

    let mut board2 = BitBoard::startpos();
    let movehist = board2.apply_pseudomoves(&[
        Square::e2.to(Square::e4).p(),
        Square::c7.to(Square::c5).p(),
        //
        Square::g1.to(Square::f3).p(),
        Square::b8.to(Square::c6).p(),
        //
        Square::c2.to(Square::c3).p(),
        Square::e7.to(Square::e6).p(),
        //
        Square::d2.to(Square::d4).p(),
        Square::d7.to(Square::d5).p(),
        //
        Square::e4.to(Square::e5).p(),
        Square::d8.to(Square::b6).p(),
        //
        Square::f1.to(Square::d3).p(),
        Square::c5.to(Square::d4).p(),
        //
        Square::e1.to(Square::g1).p(),
        Square::c8.to(Square::d7).p(),
    ]);

    assert_eq!(movehist.len(), 14);

    assert_eq!(render_fen(&board, 0), render_fen(&board2, 0));

    assert_eq!(board.white, board2.white, "WHITE");
    assert_eq!(board.black, board2.black, "BLACK");
    assert_eq!(board.metadata, board2.metadata);

    println!("X {}", show_mask(board.white.total));
    println!("x {}", show_mask(board2.white.total));

    return;

    board.enumerate(1).print();

    let mut moves = vec![];
    board.moves(&mut moves);

    let mut alg = vec![];

    for m in &moves {
        let m = *m;
        alg.push(m.ambiguate(&board, &moves).to_string());
        print!("{} ({}), ", alg.last().unwrap(), m.longalg())
    }
    println!("");

    assert!(alg.contains(&"Re1".to_string()));
    assert!(alg.contains(&"Rg1".to_string()));
}
