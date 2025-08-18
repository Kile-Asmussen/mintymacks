use crate::{
    bits::board::BitBoard,
    model::{ColorPiece, moves::Move},
};

impl BitBoard {
    pub fn apply(&mut self, mv: Move) {
        use ColorPiece::*;
        match mv.piece {
            WhitePawn => todo!(),
            BlackPawn => todo!(),
            BlackKing => todo!(),
            WhiteKing => todo!(),

            WhiteKnight => todo!(),
            WhiteBishop => todo!(),
            WhiteRook => todo!(),
            WhiteQueen => todo!(),
            BlackKnight => todo!(),
            BlackBishop => todo!(),
            BlackRook => todo!(),
            BlackQueen => todo!(),
        }
    }

    pub fn unapply(&mut self, mv: Move) {}
}
