use crate::board::{ColorPiece, Square, castling::CastlingRights};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct PseudoMove {
    pub from: Square,
    pub to: Square,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Special {
    Promotion(ColorPiece),
    CastlingOOO,
    CastlingOO,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Move {
    pub piece: ColorPiece,
    pub movement: PseudoMove,
    pub capture: Option<(ColorPiece, Square)>,
    pub special: Special,
    pub rights: CastlingRights,
    pub eps: Option<Square>,
}

#[test]
pub fn move_size() {
    println!("size: {}\nalign: {}", size_of::<Move>(), align_of::<Move>());
}
