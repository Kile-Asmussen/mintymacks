pub mod attackers;
pub mod attacks;
pub mod board;
pub mod fills;
pub mod jumps;
pub mod movegen;
pub mod moving;
pub mod rays;
pub mod slides;
pub mod tests;
pub mod victory;

use std::{num::NonZeroU64, u64};

use rand::{Rng, RngCore, SeedableRng, rngs::SmallRng};

use crate::{
    arrays::ArrayBoard,
    bits::{
        attacks::{bishop_attacks, queen_attacks, rook_attacks},
        slides::{RAYS_EAST, RAYS_NORTH, RAYS_NORTHEAST, RAYS_NORTHWEST},
    },
    model::{BoardFile, BoardRank, ChessPiece, Direction, Square, moves::PseudoMove},
};

pub type BoardMask = u64;
pub type PopulatedBoardMask = NonZeroU64;

pub struct Squares(pub BoardMask);

impl Squares {
    pub const fn next(&mut self) -> Option<Square> {
        let n = self.0.trailing_zeros();
        if n < 64 {
            self.0 &= !1 << n;
            Square::new(n as i8)
        } else {
            None
        }
    }
}

impl Iterator for Squares {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = self.0.count_ones() as usize;
        (n, Some(n))
    }
}

pub struct Bits(pub BoardMask);

impl Bits {
    pub const fn next(&mut self) -> Option<PopulatedBoardMask> {
        let n = self.0.trailing_zeros();
        if n < 64 {
            self.0 &= !1 << n;
            PopulatedBoardMask::new(1 << n)
        } else {
            None
        }
    }
}

pub const fn mask(board: [u8; 8]) -> BoardMask {
    BoardMask::from_be_bytes([
        board[0].reverse_bits(),
        board[1].reverse_bits(),
        board[2].reverse_bits(),
        board[3].reverse_bits(),
        board[4].reverse_bits(),
        board[5].reverse_bits(),
        board[6].reverse_bits(),
        board[7].reverse_bits(),
    ])
}

impl Square {
    pub const fn bit(self) -> BoardMask {
        1 << self.ix()
    }
}

pub const fn one_bit(sq: Option<Square>) -> BoardMask {
    if let Some(sq) = sq {
        sq.bit()
    } else {
        BoardMask::MIN
    }
}

impl PseudoMove {
    pub const fn bits(self) -> BoardMask {
        self.from.bit() ^ self.to.bit()
    }
}

pub const fn two_bits(mv: Option<PseudoMove>) -> BoardMask {
    if let Some(mv) = mv {
        mv.bits()
    } else {
        BoardMask::MIN
    }
}

pub fn show_mask(m: BoardMask) -> String {
    let m = m.to_be_bytes().map(u8::reverse_bits);
    format! {
"mask([
    0b_{:08b}, // 8
    0b_{:08b}, // 7
    0b_{:08b}, // 6
    0b_{:08b}, // 5
    0b_{:08b}, // 4
    0b_{:08b}, // 3
    0b_{:08b}, // 2
    0b_{:08b}, // 1
    // abcdefgh
])", m[0], m[1], m[2], m[3], m[4], m[5], m[6], m[7]}
}

impl BoardFile {
    pub const fn mask(self) -> BoardMask {
        0x0101_0101_0101_0101 << self as i8
    }
}

impl BoardRank {
    pub const fn mask(self) -> BoardMask {
        0xFF << self as i8
    }
}
