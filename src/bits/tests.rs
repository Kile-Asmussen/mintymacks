use crate::{
    bits::{
        Bits, Mask, bit,
        board::HalfBitBoard,
        mask,
        movegen::pawn_move,
        show_mask, slides,
        threats::{knight_threats, rook_threats},
    },
    model::{Color, Square, castling::CastlingRights},
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

#[test]
fn test_pawn_movegen() {
    let board = fen::parse_fen_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR").unwrap();

    let white = HalfBitBoard::new(Color::White, &board);
    let black = HalfBitBoard::new(Color::Black, &board);

    let mut moves = vec![];

    pawn_move(
        Color::White,
        &white,
        &black,
        CastlingRights::new(),
        None,
        &mut moves,
    );

    assert_eq!(moves.len(), 16);

    let mut moves = vec![];

    pawn_move(
        Color::Black,
        &black,
        &white,
        CastlingRights::new(),
        None,
        &mut moves,
    );

    assert_eq!(moves.len(), 16);
}
