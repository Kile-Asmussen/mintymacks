use crate::{
    arrays::ArrayBoard,
    model::{
        Piece, Square,
        moves::{Move, PseudoMove, Special},
    },
};

impl Square {
    pub fn str(self) -> &'static str {
        Square::STRING.at(self)
    }

    pub const STRING: ArrayBoard<&'static str> = ArrayBoard::setup([
        ["a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8"],
        ["a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7"],
        ["a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6"],
        ["a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5"],
        ["a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4"],
        ["a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3"],
        ["a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2"],
        ["a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1"],
    ]);
}

impl PseudoMove {
    pub fn longalg(self, x: Option<Piece>) -> String {
        format!(
            "{}{}{}",
            self.from.str(),
            self.to.str(),
            x.map(Piece::as_str).unwrap_or("")
        )
    }
}

impl Move {
    pub fn longalg(self) -> String {
        match self.special {
            Some(Special::Promotion(p)) => self.mv.longalg(Some(p)),
            Some(Special::Null) => "0000".to_string(),
            _ => self.mv.longalg(None),
        }
    }
}
