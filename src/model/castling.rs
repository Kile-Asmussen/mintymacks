use crate::{
    bits::{Mask, mask},
    model::{Color, File, Rank, Square, moves::PseudoMove},
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CastlingRights(u8);

impl CastlingRights {
    pub const fn new() -> Self {
        Self(0b_00001111)
    }

    pub const fn westward(self, c: Color) -> bool {
        0 != match c {
            Color::White => self.0 & 1,
            Color::Black => self.0 & 4,
        }
    }

    pub const fn eastward(self, c: Color) -> bool {
        0 != match c {
            Color::White => self.0 & 2,
            Color::Black => self.0 & 8,
        }
    }

    pub const fn lose(self, c: Color) -> Self {
        Self(match c {
            Color::White => self.0 & !3,
            Color::Black => self.0 & !12,
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CastlingDetails {
    pub capture_own_rook: bool,
    pub westward: CastlingDetail,
    pub eastward: CastlingDetail,
}

#[test]
fn castling_sizeof() {
    println!("size {}", size_of::<CastlingDetails>())
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CastlingDetail {
    pub rook_from: File,
    pub rook_to: File,
    pub king_from: File,
    pub king_to: File,
    pub threat_mask: u8,
    pub clear_mask: u8,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CastlingMove {
    pub rook_move: PseudoMove,
    pub king_move: PseudoMove,
    pub threat_mask: Mask,
    pub clear_mask: Mask,
}

impl CastlingDetail {
    pub const fn reify(self, c: Color) -> CastlingMove {
        let rank: Rank;

        match c {
            Color::White => rank = Rank::_1,
            Color::Black => rank = Rank::_8,
        };

        let rook_move = PseudoMove {
            from: self.rook_from.by(rank),
            to: self.rook_to.by(rank),
        };

        let king_move = PseudoMove {
            from: self.king_from.by(rank),
            to: self.king_to.by(rank),
        };

        let threat_mask: Mask;
        let clear_mask: Mask;

        match c {
            Color::White => {
                threat_mask = mask([0, 0, 0, 0, 0, 0, 0, self.threat_mask]);
                clear_mask = mask([0, 0, 0, 0, 0, 0, 0, self.clear_mask]);
            }
            Color::Black => {
                threat_mask = mask([self.threat_mask, 0, 0, 0, 0, 0, 0, 0]);
                clear_mask = mask([self.clear_mask, 0, 0, 0, 0, 0, 0, 0]);
            }
        }

        CastlingMove {
            rook_move,
            king_move,
            threat_mask,
            clear_mask,
        }
    }
}

pub const CLASSIC_CASTLING: CastlingDetails = CastlingDetails {
    capture_own_rook: false,
    westward: CastlingDetail {
        rook_from: File::A,
        rook_to: File::D,
        king_from: File::E,
        king_to: File::C,
        threat_mask: 0b_00111000,
        clear_mask: 0b_01110000,
    },
    eastward: CastlingDetail {
        rook_from: File::H,
        rook_to: File::F,
        king_from: File::E,
        king_to: File::G,
        threat_mask: 0b_00001110,
        clear_mask: 0b_00000110,
    },
};
