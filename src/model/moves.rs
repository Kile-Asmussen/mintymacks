use crate::model::{ColorPiece, Piece, Square, castling::CastlingRights};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct PseudoMove {
    pub from: Square,
    pub to: Square,
}

impl Square {
    pub const fn to(self, to: Square) -> PseudoMove {
        PseudoMove { from: self, to }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Special {
    Promotion(Piece),
    DrawOffer,
    CastlingWestward,
    CastlingEastward,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Move {
    pub piece: ColorPiece,
    pub mv: PseudoMove,
    pub cap: Option<(Piece, Square)>,
    pub special: Option<Special>,
    pub rights: CastlingRights,
    pub epc: Option<Square>,
}

impl Move {
    pub const fn ep_opening(self) -> Option<Square> {
        if self.piece as i8 == ColorPiece::WhitePawn as i8 {
            if (self.mv.to.ix() - self.mv.from.ix()).abs() == 16 {
                Square::new(self.mv.from.ix() + 8)
            } else {
                None
            }
        } else if self.piece as i8 == ColorPiece::BlackPawn as i8 {
            if (self.mv.to.ix() - self.mv.from.ix()).abs() == 16 {
                Square::new(self.mv.from.ix() - 8)
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[test]
pub fn move_size() {
    println!("size: {}\nalign: {}", size_of::<Move>(), align_of::<Move>());
}
