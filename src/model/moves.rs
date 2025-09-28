use crate::model::{
    Color, ColorPiece, File, Piece, Rank, Square,
    castling::{self, CastlingDetails, CastlingMove, CastlingRights},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct PseudoMove {
    pub from: Square,
    pub to: Square,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Special {
    Promotion(Piece),
    CastlingWestward,
    CastlingEastward,
    Null,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct ChessMove {
    pub piece: ColorPiece,
    pub pmv: PseudoMove,
    pub cap: Option<(Piece, Square)>,
    pub special: Option<Special>,
    pub rights: CastlingRights,
    pub epc: Option<Square>,
}

impl ChessMove {
    pub const fn ep_opening(self) -> Option<Square> {
        if self.piece as i8 == ColorPiece::WhitePawn as i8 {
            if (self.pmv.to.ix() - self.pmv.from.ix()).abs() == 16 {
                Square::new(self.pmv.from.ix() + 8)
            } else {
                None
            }
        } else if self.piece as i8 == ColorPiece::BlackPawn as i8 {
            if (self.pmv.to.ix() - self.pmv.from.ix()).abs() == 16 {
                Square::new(self.pmv.from.ix() - 8)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub const fn simplify(self) -> (PseudoMove, Option<Piece>) {
        (
            self.pmv,
            match self.special {
                Some(Special::Promotion(p)) => Some(p),
                _ => None,
            },
        )
    }

    pub const fn castling_change(self, details: CastlingDetails) -> CastlingRights {
        use ColorPiece::*;
        let mut rights = self.rights;

        rights = match self.piece {
            WhiteKing | BlackKing => rights.move_king(self.piece.color()),
            WhiteRook => move_rook(self.pmv.from, Color::White, details, rights),
            BlackRook => move_rook(self.pmv.from, Color::Black, details, rights),
            _ => rights,
        };

        rights = if let Some((Piece::Rook, sq)) = self.cap {
            move_rook(sq, self.piece.color().opposite(), details, rights)
        } else {
            rights
        };

        #[must_use]
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
    println!(
        "size: {}\nalign: {}",
        size_of::<ChessMove>(),
        align_of::<ChessMove>()
    );
    println!("option-size: {}", size_of::<Option<ChessMove>>());
}
