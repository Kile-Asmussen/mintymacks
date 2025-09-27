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
            x.map(Piece::as_str).unwrap_or("")
        )
    }
}

impl ChessMove {
    pub fn longalg(self) -> String {
        match self.special {
            Some(Special::Promotion(p)) => self.mv.longalg(Some(p)),
            Some(Special::Null) => "0000".to_string(),
            _ => self.mv.longalg(None),
        }
    }
}

pub fn parse_longalg(s: &str) -> Option<(PseudoMove, Option<Piece>)> {
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
