use std::str::Chars;

use crate::regex;

use crate::model::{
    File, Piece, Rank, Square,
    moves::{ChessMove, PseudoMove, Special},
};

impl PseudoMove {
    pub fn longalg(self, x: Option<Piece>) -> String {
        format!(
            "{}{}{}",
            self.from.to_str(),
            self.to.to_str(),
            x.map(|x| match x {
                Piece::Pawn => "p",
                Piece::Knight => "n",
                Piece::Bishop => "b",
                Piece::Rook => "r",
                Piece::Queen => "q",
                Piece::King => "k",
            })
            .unwrap_or("")
        )
    }

    pub const fn q(self) -> (Self, Option<Piece>) {
        (self, Some(Piece::Queen))
    }

    pub const fn r(self) -> (Self, Option<Piece>) {
        (self, Some(Piece::Rook))
    }

    pub const fn b(self) -> (Self, Option<Piece>) {
        (self, Some(Piece::Bishop))
    }

    pub const fn n(self) -> (Self, Option<Piece>) {
        (self, Some(Piece::Knight))
    }

    pub const fn p(self) -> (Self, Option<Piece>) {
        (self, None)
    }

    pub fn parse(s: &str) -> Option<(PseudoMove, Option<Piece>)> {
        let cs = regex!("([a-h][1-8])([a-h][1-8])([nbrq])?").captures(s)?;
        let org = Square::from_str(&cs[1])?;
        let dst = Square::from_str(&cs[2])?;
        let prom = match &cs[3] {
            "n" => Some(Piece::Knight),
            "b" => Some(Piece::Bishop),
            "r" => Some(Piece::Rook),
            "q" => Some(Piece::Queen),
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
        match self.special {
            Some(Special::Promotion(p)) => self.pmv.longalg(Some(p)),
            Some(Special::Null) => "0000".to_string(),
            _ => self.pmv.longalg(None),
        }
    }
}
