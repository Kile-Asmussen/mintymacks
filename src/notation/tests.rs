use crate::{
    arrays::ArrayBoard,
    bits::board::BitBoard,
    model::{
        ColorPiece, File, Piece, Square,
        moves::{PseudoMove, Special},
    },
    notation::{
        algebraic::AlgebraicMove,
        fen::{parse_fen, parse_fen_board, render_fen_board},
    },
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

    let startpos = ArrayBoard::setup([
        [
            BlackRook,
            BlackKnight,
            BlackBishop,
            BlackQueen,
            BlackKing,
            BlackBishop,
            BlackKnight,
            BlackRook,
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
            WhiteRook,
        ]
        .map(Some),
    ]);

    assert_eq!(
        parse_fen_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR").unwrap(),
        startpos
    );

    assert_eq!(
        render_fen_board(&startpos),
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"
    );

    assert_eq!(
        BitBoard::startpos(),
        parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap(),
    );
}

#[test]
fn longalg_roundtrips() {
    assert_eq!(
        Some(Square::a7.to(Square::a8).q()),
        PseudoMove::parse(&Square::a7.to(Square::a8).longalg(Some(Piece::Queen)))
    );

    assert_eq!("a7a8q", {
        let (pmv, pr) = PseudoMove::parse("a7a8q").unwrap();
        pmv.longalg(pr)
    })
}

#[test]
fn algebraic_roundtrip() {
    assert_eq!(
        Some(AlgebraicMove {
            piece: Piece::Pawn,
            file_origin: Some(File::E),
            rank_origin: None,
            destination: Square::f8,
            capture: true,
            special: Some(Special::Promotion(Piece::Rook)),
            check_or_mate: Some(false)
        }),
        AlgebraicMove::parse("exf8=R+")
    );
}
