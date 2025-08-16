use crate::model::{Color, ColorPiece, File, Piece, Rank, Square};

#[cfg(test)]
fn has_null_optimization<T>() {
    assert_eq!(std::mem::size_of::<Option<T>>(), std::mem::size_of::<T>());
}

#[test]
fn nullopt() {
    has_null_optimization::<Square>();
    has_null_optimization::<Color>();
    has_null_optimization::<Piece>();
    has_null_optimization::<ColorPiece>();
    has_null_optimization::<Rank>();
    has_null_optimization::<File>();
}
