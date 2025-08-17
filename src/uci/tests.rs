use crate::{
    arrays::ArrayBoard,
    model::ColorPiece,
    uci::fen::{parse_fen_board, render_fen_board},
};

#[test]
fn fen_board_roundtrip() {
    assert_eq!(
        parse_fen_board("8/8/8/8/8/8/8/8").unwrap(),
        ArrayBoard::new(None)
    );

    assert_eq!(
        render_fen_board(&parse_fen_board("8/8/8/8/8/8/8/8").unwrap()),
        "8/8/8/8/8/8/8/8".to_string()
    );

    use ColorPiece::*;

    assert_eq!(
        parse_fen_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR").unwrap(),
        ArrayBoard::setup([
            [
                BlackRook,
                BlackKnight,
                BlackBishop,
                BlackQueen,
                BlackKing,
                BlackBishop,
                BlackKnight,
                BlackRook
            ]
            .map(Some),
            [BlackPawn; 8].map(Some),
            [None; 8],
            [None; 8],
            [None; 8],
            [None; 8],
            [WhitePawn; 8].map(Some),
            [
                WhiteRook,
                WhiteKnight,
                WhiteBishop,
                WhiteQueen,
                WhiteKing,
                WhiteBishop,
                WhiteKnight,
                WhiteRook
            ]
            .map(Some)
        ])
    );
}
