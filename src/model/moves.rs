use crate::model::{
    Color, ColorPiece, Piece, Square,
    castling::{self, CastlingDetails, CastlingRights},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct PseudoMove {
    pub from: Square,
    pub to: Square,
}

impl Square {
    pub const fn to(self, to: Square) -> PseudoMove {
        PseudoMove { from: self, to }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Special {
    Promotion(Piece),
    CastlingWestward,
    CastlingEastward,
    Null,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
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

    pub const fn matches(self, mv: PseudoMove, prom: Option<Piece>) -> bool {
        self.mv.from.ix() == mv.from.ix()
            && self.mv.to.ix() == mv.to.ix()
            && match (self.special, prom) {
                (Some(Special::Promotion(p)), Some(p2)) => p as i8 == p2 as i8,
                (Some(Special::Promotion(_)), None) => false,
                (_, None) => true,
                (_, Some(_)) => false,
            }
    }

    pub const fn simplify(self) -> (PseudoMove, Option<Piece>) {
        (
            self.mv,
            match self.special {
                Some(Special::Promotion(p)) => Some(p),
                _ => None,
            },
        )
    }

    pub const fn castling_change(self, details: CastlingDetails) -> CastlingRights {
        use ColorPiece::*;
        let mut rights = self.rights;

        match self.piece {
            WhiteKing | BlackKing => rights.move_king(self.piece.color()),
            WhiteRook => move_rook(self.mv.from, Color::White, details, rights),
            BlackRook => move_rook(self.mv.from, Color::Black, details, rights),
            _ => rights,
        };

        rights = if let Some((Piece::Rook, sq)) = self.cap {
            move_rook(sq, self.piece.color().opposite(), details, rights)
        } else {
            rights
        };

        const fn move_rook(
            from: Square,
            color: Color,
            details: CastlingDetails,
            rights: CastlingRights,
        ) -> CastlingRights {
            if from.ix() == details.eastward.rook_from.by(color.rank()).ix() {
                rights.move_east_rook(color)
            } else if from.ix() == details.westward.rook_from.by(color.rank()).ix() {
                rights.move_west_rook(color)
            } else {
                rights
            }
        }

        rights
    }
}

#[test]
pub fn move_size() {
    println!("size: {}\nalign: {}", size_of::<Move>(), align_of::<Move>());
    println!("option-size: {}", size_of::<Option<Move>>());
}
