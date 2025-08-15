mod board;
mod slides;
mod tests;

use crate::board::{File, Rank, Square, Squares};

pub type Mask = u64;

pub struct Bits(pub Mask);

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

pub const fn mask(board: [u8; 8]) -> Mask {
    Mask::from_be_bytes([
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

pub const fn bit(sq: Square) -> Mask {
    1 << sq.ix()
}

pub const fn slide_move_stop_positive(
    move_mask: Mask,
    uncapturable: Mask,
    capturable: Mask,
) -> Mask {
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
    move_mask: Mask,
    uncapturable: Mask,
    capturable: Mask,
) -> Mask {
    slide_move_stop_positive(
        move_mask.reverse_bits(),
        uncapturable.reverse_bits(),
        capturable.reverse_bits(),
    )
    .reverse_bits()
}
