use strum::VariantArray;

use crate::model::{
    BoardFile, BoardRank, ChessPiece, Color, ColoredChessPiece, ColoredChessPieceWithCapture,
    Square, moves::PseudoMove,
};

#[cfg(test)]
fn has_null_optimization<T>() {
    assert_eq!(std::mem::size_of::<Option<T>>(), std::mem::size_of::<T>());
}

#[test]
fn nullopt() {
    has_null_optimization::<Square>();
    has_null_optimization::<Color>();
    has_null_optimization::<ChessPiece>();
    has_null_optimization::<ColoredChessPiece>();
    has_null_optimization::<BoardRank>();
    has_null_optimization::<BoardFile>();
    has_null_optimization::<PseudoMove>();
}

#[test]
fn colored_chess_piece_with_capture() {
    for cp in ColoredChessPiece::VARIANTS {
        let cp = *cp;
        for cap in ChessPiece::VARIANTS {
            let cap = *cap;
            if cap == ChessPiece::King {
                continue;
            }
            let cp_cap = ColoredChessPieceWithCapture::new(cp, Some(cap));
            assert_eq!(cp, cp_cap.color_piece(), "{:?} {:?}", cp, cap);
            assert_eq!(Some(cap), cp_cap.capture(), "{:?} {:?}", cp, cap);
        }
        let cp_cap = ColoredChessPieceWithCapture::new(cp, None);
        assert_eq!(cp, cp_cap.color_piece(), "{:?} None", cp);
        assert_eq!(None, cp_cap.capture(), "{:?} None", cp);
    }
}
