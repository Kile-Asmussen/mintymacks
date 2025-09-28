pub mod board;
pub mod jumps;
pub mod movegen;
pub mod moving;
pub mod slides;
pub mod tests;
pub mod threats;

use crate::model::{File, Rank, Square, moves::PseudoMove};

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

pub const fn slide_move_stop_positive(
    move_mask: BoardMask,
    uncapturable: BoardMask,
    capturable: BoardMask,
) -> BoardMask {
    let uncapturable_on_move_mask = move_mask & uncapturable;
    let capturable_on_move_mask = move_mask & capturable;

    let allowed_by_uncapturable =
        move_mask & ((uncapturable_on_move_mask.wrapping_sub(1)) & !uncapturable_on_move_mask);
    let allowed_by_capturable =
        move_mask & ((capturable_on_move_mask.wrapping_sub(1)) ^ capturable_on_move_mask);
    let allowed = allowed_by_capturable & allowed_by_uncapturable;

    allowed
}

pub const fn slide_move_stop_negative(
    move_mask: BoardMask,
    uncapturable: BoardMask,
    capturable: BoardMask,
) -> BoardMask {
    slide_move_stop_positive(
        move_mask.reverse_bits(),
        uncapturable.reverse_bits(),
        capturable.reverse_bits(),
    )
    .reverse_bits()
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
