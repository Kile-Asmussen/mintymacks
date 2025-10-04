pub mod board;
pub mod jumps;
pub mod movegen;
pub mod moving;
pub mod slides;
pub mod tests;
pub mod threats;
pub mod victory;
pub mod movecount;

use rand::{rngs::SmallRng, Rng, RngCore, SeedableRng};

use crate::{arrays::ArrayBoard, bits::slides::{RAYS_EAST, RAYS_NORTH, RAYS_NORTHEAST, RAYS_NORTHWEST}, model::{moves::PseudoMove, BoardFile, BoardRank, ChessPiece, Square}};

pub type BoardMask = u64;

pub struct Bits(pub BoardMask);

impl Bits {
    pub const fn next(&mut self) -> Option<Square> {
        let n = self.0.trailing_zeros();
        if n != 64 {
            self.0 &= !1 << n;
        }
        Square::new(n as i8)
    }
}

impl Iterator for Bits {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = self.0.count_ones() as usize;
        (n, Some(n))
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

pub const fn bit(sq: Option<Square>) -> BoardMask {
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

#[inline]
pub const fn slide_move_attacks(
    neg_ray: BoardMask,
    pos_ray: BoardMask,
    occupied: BoardMask
) -> BoardMask {
    let neg_hit = neg_ray & occupied;
    let pos_hit = pos_ray & occupied;
    let ms1b = 1u64 << (63 - (neg_hit & occupied | 1).leading_zeros());
    let diff = pos_hit ^ pos_hit.wrapping_sub(ms1b);
    return (neg_ray | pos_ray) & diff;
}

#[test]
fn square_iter() {
    assert_eq!(Bits(1).next(), Some(Square::a1));
    assert_eq!(Bits(2).next(), Some(Square::b1));
    assert_eq!(Bits(3).collect::<Vec<_>>(), vec![Square::a1, Square::b1]);
    assert_eq!(Bits(6).collect::<Vec<_>>(), vec![Square::b1, Square::c1]);
}

#[test]
#[rustfmt::skip]
fn mask_board_setup() {
    assert_eq!(
        Bits(mask([
            0b_00000000, // 8
            0b_00000000, // 7
            0b_00000000, // 6
            0b_00000000, // 5
            0b_00000000, // 4
            0b_00000000, // 3
            0b_00000000, // 2
            0b_10000000, // 1
            // abcdefgh
        ]))
        .next(),
        Some(Square::a1)
    );

    assert_eq!(
        Square::a1.bit(),
        mask([
            0b_00000000, // 8
            0b_00000000, // 7
            0b_00000000, // 6
            0b_00000000, // 5
            0b_00000000, // 4
            0b_00000000, // 3
            0b_00000000, // 2
            0b_10000000, // 1
            // abcdefgh
        ])
    );

    assert_eq!(
        Bits(mask([
            0b_00000001, // 8
            0b_00000000, // 7
            0b_00000000, // 6
            0b_00000000, // 5
            0b_00000000, // 4
            0b_00000000, // 3
            0b_00000000, // 2
            0b_00000000, // 1
            // abcdefgh
        ]))
        .next(),
        Some(Square::h8)
    );
}
