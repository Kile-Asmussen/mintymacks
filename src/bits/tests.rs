use crate::{
    bits::{
        Bits, Mask, bit,
        board::{BitMetadata, HalfBitBoard},
        mask,
        movegen::{legal_moves, pawn_moves},
        show_mask, slides,
        threats::{knight_threats, rook_threats},
    },
    model::{
        Color, Square,
        castling::{CLASSIC_CASTLING, CastlingRights},
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

#[test]
fn test_movegen() {
    let board = fen::parse_fen_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR").unwrap();

    let white = HalfBitBoard::new(Color::White, &board);
    let black = HalfBitBoard::new(Color::Black, &board);
    let metadata = BitMetadata {
        to_move: Color::White,
        castling_rights: CastlingRights::new(),
        en_passant: None,
        castling_details: CLASSIC_CASTLING,
    };

    let mut moves = vec![];

    legal_moves(&white, &black, metadata, &mut moves);

    assert_eq!(moves.len(), 20);

    let board = fen::parse_fen_board("R6R/3Q4/1Q4Q1/4Q3/2Q4Q/Q4Q2/pp1Q4/kBNN1KB1").unwrap();

    let white = HalfBitBoard::new(Color::White, &board);
    let black = HalfBitBoard::new(Color::Black, &board);
    let metadata = BitMetadata {
        to_move: Color::White,
        castling_rights: CastlingRights::new()
            .move_king(Color::White)
            .move_king(Color::Black),
        en_passant: None,
        castling_details: CLASSIC_CASTLING,
    };

    let mut moves = vec![];

    legal_moves(&white, &black, metadata, &mut moves);

    assert_eq!(moves.len(), 218);
}
