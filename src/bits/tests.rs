use crate::{
    bits::{
        Bits, Mask, bit,
        board::{BitBoard, HalfBitBoard},
        mask,
        movegen::{legal_moves, pawn_moves},
        show_mask, slides,
        threats::{knight_threats, rook_threats},
    },
    model::{
        Color, ColorPiece, Square,
        castling::{CLASSIC_CASTLING, CastlingRights},
        metadata::Metadata,
        moves::{Move, PseudoMove},
    },
    uci::fen,
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

fn test_move_numbers(fen: &str, c: Color, cr: CastlingRights, epc: Option<Square>, num: usize) {
    let board = fen::parse_fen_board(fen).unwrap();

    let white = HalfBitBoard::new(Color::White, &board);
    let black = HalfBitBoard::new(Color::Black, &board);
    let metadata = Metadata {
        to_move: c,
        castling_rights: cr,
        en_passant: epc,
        castling_details: CLASSIC_CASTLING,
    };

    let mut moves = vec![];
    legal_moves(&white, &black, metadata, &mut moves);
    assert_eq!(moves.len(), num);
}

#[test]
fn test_movegen() {
    test_move_numbers(
        "8/8/8/8/8/8/8/8",
        Color::White,
        CastlingRights::new(),
        None,
        0,
    );

    test_move_numbers(
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR",
        Color::White,
        CastlingRights::new(),
        None,
        20,
    );

    test_move_numbers(
        "R6R/3Q4/1Q4Q1/4Q3/2Q4Q/Q4Q2/pp1Q4/kBNN1KB1",
        Color::White,
        CastlingRights::new(),
        None,
        218,
    );
}

#[test]
fn test_moving() {
    let mut board = BitBoard::startpos();
    board.apply(Move {
        piece: ColorPiece::WhitePawn,
        mv: Square::d2.to(Square::d4),
        cap: None,
        special: None,
        rights: CastlingRights::new(),
        epc: None,
    });

    assert_eq!(
        board.render(),
        fen::parse_fen_board("rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR").unwrap()
    );
}
