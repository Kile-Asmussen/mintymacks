use crate::board::{
    CastlingMove, CastlingRights, Color, ColorPiece, File, Piece, Rank, Square, Squares,
};

#[test]
fn squares_iter() {
    assert_eq!(Squares::all().collect::<Vec<_>>().len(), 64);
}

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
    has_null_optimization::<CastlingMove>();
    has_null_optimization::<CastlingRights>();
}
