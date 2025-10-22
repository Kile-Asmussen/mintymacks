use rand::RngCore;

use crate::{
    arrays::ArrayBoard,
    bits::{
        BoardMask, Squares,
        attacks::{bishop_attacks, queen_attacks, rook_attacks},
        show_mask,
        slides::{
            obstruction_difference, simple_diagonal_attack, simple_omnidirectional_attack,
            simple_orthogonal_attack,
        },
    },
    fuzzing::pi_rng,
    model::{Direction, Square},
};

#[inline]
pub const fn difference_obstruction(
    neg_occ: BoardMask,
    pos_occ: BoardMask,
    ray: BoardMask,
) -> BoardMask {
    let neg_hit = ray & neg_occ;
    let pos_hit = ray & pos_occ;
    let ms1b = 1u64 << (63 - (neg_hit & (neg_occ | pos_occ) | 1).leading_zeros());
    let diff = pos_hit ^ pos_hit.wrapping_sub(ms1b);
    return ray & diff;
}

#[inline]
pub fn rook_lasers(rooks: BoardMask, occupied: BoardMask) -> BoardMask {
    let mut res = 0;
    for sq in Squares(rooks) {
        res |= rook_rays(sq, occupied);
    }
    res
}

#[inline]
pub fn bishop_lasers(rooks: BoardMask, occupied: BoardMask) -> BoardMask {
    let mut res = 0;
    for sq in Squares(rooks) {
        res |= bishop_rays(sq, occupied);
    }
    res
}

#[inline]
pub fn queen_lasers(rooks: BoardMask, occupied: BoardMask) -> BoardMask {
    let mut res = 0;
    for sq in Squares(rooks) {
        res |= queen_rays(sq, occupied);
    }
    res
}

#[inline]
pub fn rook_rays(sq: Square, occupied: BoardMask) -> BoardMask {
    let (neg_occ, pos_occ) = split(sq, occupied);
    let rank = difference_obstruction(neg_occ, pos_occ, rank_ray(sq));
    let file = difference_obstruction(neg_occ, pos_occ, file_ray(sq));
    (rank | file) & !sq.bit()
}

#[inline]
pub fn bishop_rays(sq: Square, occupied: BoardMask) -> BoardMask {
    let (neg_occ, pos_occ) = split(sq, occupied);
    let diag = difference_obstruction(neg_occ, pos_occ, diag_ray(sq));
    let anti = difference_obstruction(neg_occ, pos_occ, anti_ray(sq));
    (diag | anti) & !sq.bit()
}

#[inline]
pub fn queen_rays(sq: Square, occupied: BoardMask) -> BoardMask {
    let (neg_occ, pos_occ) = split(sq, occupied);
    let rank = difference_obstruction(neg_occ, pos_occ, rank_ray(sq));
    let file = difference_obstruction(neg_occ, pos_occ, file_ray(sq));
    let diag = difference_obstruction(neg_occ, pos_occ, diag_ray(sq));
    let anti = difference_obstruction(neg_occ, pos_occ, anti_ray(sq));
    (rank | file | diag | anti) & !sq.bit()
}

#[inline]
pub fn rank_ray(sq: Square) -> BoardMask {
    0x0000_0000_0000_00FF << (sq.ix() & 0x38)
}

#[inline]
pub fn file_ray(sq: Square) -> BoardMask {
    0x0101_0101_0101_0101 << (sq.ix() & 0x7)
}

/// Diagonal intersecting a square
#[inline]
pub fn diag_ray(sq: Square) -> BoardMask {
    let mut diag = 0x8040_2010_0804_0201u128;
    (diag << (64 + (sq.ix() & 0x38) - ((sq.ix() & 0x7) << 3)) >> 64) as u64
}

/// Anti-diagonal intersecting a square
#[inline]
pub fn anti_ray(sq: Square) -> BoardMask {
    let mut diag = 0x0102_0408_1020_4080u128;
    (diag << (8 + (sq.ix() & 0x38) + ((sq.ix() & 0x7) << 3)) >> 64) as u64
}

#[inline]
pub fn split(sq: Square, mask: BoardMask) -> (BoardMask, BoardMask) {
    let (lo, hi) = (!0 >> (63 - sq.ix()) & !sq.bit(), !0 << sq.ix() & !sq.bit());
    (lo & mask, hi & mask)
}

#[test]
fn test_rook_rays() {
    println!("{}", show_mask(rook_rays(Square::d4, 0)));
}

#[test]
fn test_rays() {
    let mut rng = pi_rng();

    for _ in 0..100 {
        let total = rng.next_u64() & rng.next_u64();
        for sq1 in Squares(u64::MAX) {
            let total = total | sq1.bit();

            assert_eq!(rook_rays(sq1, total), simple_orthogonal_attack(sq1, total));
            assert_eq!(bishop_rays(sq1, total), simple_diagonal_attack(sq1, total));
            assert_eq!(
                queen_rays(sq1, total),
                simple_omnidirectional_attack(sq1, total)
            );
        }
    }
}
