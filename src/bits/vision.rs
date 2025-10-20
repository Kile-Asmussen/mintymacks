use crate::{
    arrays::ArrayBoard,
    bits::{
        BoardMask, Squares,
        jumps::{BLACK_PAWN_CAPTURE, WHITE_PAWN_CAPTURE},
        slides::{BLACK_PAWN_MOVES, WHITE_PAWN_MOVES, obstruction_difference},
    },
    model::{Color, Square},
};

pub trait PieceVision {
    fn new(total: BoardMask) -> Self;

    #[inline]
    fn see(&self, sq: Square) -> BoardMask;

    #[inline]
    fn surveil(&self, pieces: BoardMask) -> BoardMask {
        let mut res = 0;
        for sq in Squares(pieces) {
            res |= self.see(sq);
        }
        res
    }
}

pub trait PawnVision {
    fn new(color: Color, total: BoardMask) -> Self;

    #[inline]
    fn see(&self, sq: Square) -> BoardMask;

    #[inline]
    fn run(&self, sq: Square) -> BoardMask;

    #[inline]
    fn surveil(&self, pieces: BoardMask) -> BoardMask {
        let mut res = 0;
        for sq in Squares(pieces) {
            res |= self.see(sq);
        }
        res
    }
}
