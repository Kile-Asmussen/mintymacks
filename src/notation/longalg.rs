use std::str::Chars;

use crate::model::{
    BoardFile, BoardRank, ChessPiece, Square,
    moves::{ChessMove, PseudoMove, SpecialMove},
};
use crate::regexp;

impl PseudoMove {
    pub fn longalg(self, x: Option<ChessPiece>) -> String {
        format!(
            "{}{}{}",
            self.from.to_str(),
            self.to.to_str(),
            x.map(|x| match x {
                ChessPiece::Pawn => "p",
                ChessPiece::Knight => "n",
                ChessPiece::Bishop => "b",
                ChessPiece::Rook => "r",
                ChessPiece::Queen => "q",
                ChessPiece::King => "k",
            })
            .unwrap_or("")
        )
    }

    pub const fn q(self) -> (Self, Option<ChessPiece>) {
        (self, Some(ChessPiece::Queen))
    }

    pub const fn r(self) -> (Self, Option<ChessPiece>) {
        (self, Some(ChessPiece::Rook))
    }

    pub const fn b(self) -> (Self, Option<ChessPiece>) {
        (self, Some(ChessPiece::Bishop))
    }

    pub const fn n(self) -> (Self, Option<ChessPiece>) {
        (self, Some(ChessPiece::Knight))
    }

    pub const fn p(self) -> (Self, Option<ChessPiece>) {
        (self, None)
    }

    pub fn parse(s: &str) -> Option<(PseudoMove, Option<ChessPiece>)> {
        let cs = regexp!("([a-h][1-8])([a-h][1-8])([nbrq]?)").captures(s)?;
        let org = Square::parse(&cs[1])?;
        let dst = Square::parse(&cs[2])?;
        let prom = match &cs[3] {
            "n" => Some(ChessPiece::Knight),
            "b" => Some(ChessPiece::Bishop),
            "r" => Some(ChessPiece::Rook),
            "q" => Some(ChessPiece::Queen),
            "" => None,
            _ => return None,
        };

        Some((org.to(dst), prom))
    }
}

impl Square {
    pub const fn to(self, to: Square) -> PseudoMove {
        PseudoMove { from: self, to }
    }
}

impl ChessMove {
    pub fn longalg(self) -> String {
        match self.spc {
            Some(SpecialMove::Promotion(p)) => self.pmv.longalg(Some(p)),
            Some(SpecialMove::Null) => "0000".to_string(),
            _ => self.pmv.longalg(None),
        }
    }
}
