use crate::model::{Color, ColoredChessPiece, BoardFile, ChessPiece, BoardRank, Square, moves::PseudoMove};

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
