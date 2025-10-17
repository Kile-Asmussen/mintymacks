use rand::RngCore;

use crate::{
    arrays::ArrayBoard,
    bits::{
        BoardMask, Squares,
        attacks::{bishop_attacks, queen_attacks, rook_attacks},
        show_mask,
    },
    fuzzing::test::pi_rng,
    model::{Direction, Square},
};

#[inline]
pub const fn obstruction_difference(
    neg_ray: BoardMask,
    pos_ray: BoardMask,
    occupied: BoardMask,
) -> BoardMask {
    let neg_hit = neg_ray & occupied;
    let pos_hit = pos_ray & occupied;
    let ms1b = 1u64 << (63 - (neg_hit & occupied | 1).leading_zeros());
    // let ms1b = 1u64 << neg_hit.checked_ilog2().unwrap_or(0);
    let diff = pos_hit ^ pos_hit.wrapping_sub(ms1b);
    return (neg_ray | pos_ray) & diff;
}

#[inline]
pub const fn difference_obstruction(
    occupied: BoardMask,
    neg_occ: BoardMask,
    pos_occupied: BoardMask,
    ray: BoardMask,
) -> BoardMask {
    let neg_hit = ray & neg_occ;
    let pos_hit = ray & pos_occupied;
    let ms1b = 1u64 << (63 - (neg_hit & occupied | 1).leading_zeros());
    // let ms1b = 1u64 << neg_hit.checked_ilog2().unwrap_or(0);
    let diff = pos_hit ^ pos_hit.wrapping_sub(ms1b);
    return ray & diff;
}

#[inline]
pub const fn rook_rays(sq: Square, occupied: BoardMask) -> BoardMask {
    let (neg_occ, pos_occ) = split(sq, occupied);
    let rank = difference_obstruction(occupied, neg_occ, pos_occ, rank_ray(sq));
    let file = difference_obstruction(occupied, neg_occ, pos_occ, file_ray(sq));
    (rank | file) & !sq.bit()
}

#[inline]
pub const fn bishop_rays(sq: Square, occupied: BoardMask) -> BoardMask {
    let (neg_occ, pos_occ) = split(sq, occupied);
    let diag = difference_obstruction(occupied, neg_occ, pos_occ, diag_ray(sq));
    let anti = difference_obstruction(occupied, neg_occ, pos_occ, anti_ray(sq));
    (diag | anti) & !sq.bit()
}

#[inline]
pub const fn queen_rays(sq: Square, occupied: BoardMask) -> BoardMask {
    let (neg_occ, pos_occ) = split(sq, occupied);
    let rank = difference_obstruction(occupied, neg_occ, pos_occ, rank_ray(sq));
    let file = difference_obstruction(occupied, neg_occ, pos_occ, file_ray(sq));
    let diag = difference_obstruction(occupied, neg_occ, pos_occ, diag_ray(sq));
    let anti = difference_obstruction(occupied, neg_occ, pos_occ, anti_ray(sq));
    (rank | file | diag | anti) & !sq.bit()
}

/// Anti-diagonal intersecting a square
#[inline]
pub const fn rank_ray(sq: Square) -> BoardMask {
    0x0000_0000_0000_00FF << (sq.ix() & 0x38)
}

/// Anti-diagonal intersecting a square
#[inline]
pub const fn file_ray(sq: Square) -> BoardMask {
    0x0101_0101_0101_0101 << (sq.ix() & 0x7)
}

#[inline]
pub const fn diag_ray(sq: Square) -> BoardMask {
    let mut diag = 0x8040_2010_0804_0201u128;
    (diag << (64 + (sq.ix() & 0x38) - ((sq.ix() & 0x7) << 3)) >> 64) as u64
}

#[inline]
pub const fn anti_ray(sq: Square) -> BoardMask {
    let mut diag = 0x8040_2010_0804_0201u128;
    (diag << (8 + (sq.ix() & 0x38) + ((sq.ix() & 0x7) << 3)) >> 64) as u64
}

pub const fn split(sq: Square, mask: BoardMask) -> (BoardMask, BoardMask) {
    let (lo, hi) = (!0 >> (63 - sq.ix()) & !sq.bit(), !0 << sq.ix() & !sq.bit());
    (lo & mask, hi & mask)
}

#[test]
fn test_rays() {
    let mut rng = pi_rng();
}
