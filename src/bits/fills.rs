use std::{
    simd::{Simd, num::SimdUint},
    time::Instant,
};

use rand::RngCore;

use crate::{
    bits::{
        Bits, BoardMask, Squares,
        attacks::{
            bishop_attacks, king_attacks, knight_attacks, pawn_attacks, queen_attacks, rook_attacks,
        },
        mask,
        movegen::pawn_moves,
        show_mask,
        slides::{BLACK_PAWN_MOVES, WHITE_PAWN_MOVES, obstruction_difference},
    },
    fuzzing::test::pi_rng,
    model::{BoardFile, BoardRank, Color, Direction, Square},
};

type Qu64 = Simd<u64, 4>;
type Du64 = Simd<u64, 2>;

pub fn rook_fill(mask: u64, empty: u64) -> u64 {
    return rook_upper(mask, empty) | rook_lower(mask, empty);

    const SHIFT: [u64; 2] = [Direction::North as u64, Direction::East as u64];

    #[inline]
    fn rook_upper(mask: u64, empty: u64) -> u64 {
        const MASK: [u64; 2] = [!0, !BoardFile::A.mask()];

        let empty = Du64::from_array([empty; 2]) & Du64::from_array(MASK);
        let mut prop = Du64::from_array([mask; 2]);
        let mut fill = Du64::from_array([mask; 2]);

        for _ in 0..5 {
            prop = prop << Du64::from_array(SHIFT) & empty;
            fill |= prop;
        }

        fill |= (prop << Du64::from_array(SHIFT)) & empty;

        (fill << Du64::from_array(SHIFT) & Du64::from_array(MASK)).reduce_or()
    }

    #[inline]
    fn rook_lower(mask: u64, empty: u64) -> u64 {
        const MASK: [u64; 2] = [!0, !BoardFile::H.mask()];

        let empty = Du64::from_array([empty; 2]) & Du64::from_array(MASK);
        let mut prop = Du64::from_array([mask; 2]);
        let mut fill = Du64::from_array([mask; 2]);

        for _ in 0..5 {
            prop = prop >> Du64::from_array(SHIFT) & empty;
            fill |= prop;
        }

        fill |= (prop >> Du64::from_array(SHIFT)) & empty;

        (fill >> Du64::from_array(SHIFT) & Du64::from_array(MASK)).reduce_or()
    }
}

pub fn bishop_fill(mask: u64, empty: u64) -> u64 {
    return bishop_upper(mask, empty) | bishop_lower(mask, empty);

    const SHIFT: [u64; 2] = [Direction::NorthWest as u64, Direction::NorthEast as u64];

    #[inline]
    fn bishop_upper(mask: u64, empty: u64) -> u64 {
        const MASK: [u64; 2] = [!BoardFile::H.mask(), !BoardFile::A.mask()];

        let empty = Du64::from_array([empty; 2]) & Du64::from_array(MASK);
        let mut prop = Du64::from_array([mask; 2]);
        let mut fill = Du64::from_array([mask; 2]);

        for _ in 0..5 {
            prop = prop << Du64::from_array(SHIFT) & empty;
            fill |= prop;
        }

        fill |= (prop << Du64::from_array(SHIFT)) & empty;

        (fill << Du64::from_array(SHIFT) & Du64::from_array(MASK)).reduce_or()
    }

    #[inline]
    fn bishop_lower(mask: u64, empty: u64) -> u64 {
        const MASK: [u64; 2] = [!BoardFile::A.mask(), !BoardFile::H.mask()];
        let empty = Du64::from_array([empty; 2]) & Du64::from_array(MASK);
        let mut prop = Du64::from_array([mask; 2]);
        let mut fill = Du64::from_array([mask; 2]);

        for _ in 0..5 {
            prop = prop >> Du64::from_array(SHIFT) & empty;
            fill |= prop;
        }

        fill |= (prop >> Du64::from_array(SHIFT)) & empty;

        (fill >> Du64::from_array(SHIFT) & Du64::from_array(MASK)).reduce_or()
    }
}

#[cfg(target_arch = "x86_64")]
pub fn queen_fill(mask: u64, empty: u64) -> u64 {
    return queen_upper(mask, empty) | queen_lower(mask, empty);

    const SHIFT: [u64; 4] = [
        Direction::NorthWest as u64,
        Direction::North as u64,
        Direction::NorthEast as u64,
        Direction::East as u64,
    ];

    #[inline]
    fn queen_upper(mask: u64, empty: u64) -> u64 {
        const MASK: [u64; 4] =
            [!BoardFile::H.mask(), !0, !BoardFile::A.mask(), !BoardFile::A.mask()];
        let empty = Qu64::from_array([empty; 4]) & Qu64::from_array(MASK);
        let mut prop = Qu64::from_array([mask; 4]);
        let mut fill = Qu64::from_array([mask; 4]);

        for _ in 0..5 {
            prop = prop << Qu64::from_array(SHIFT) & empty;
            fill |= prop;
        }

        fill |= (prop << Qu64::from_array(SHIFT)) & empty;

        (fill << Qu64::from_array(SHIFT) & Qu64::from_array(MASK)).reduce_or()
    }

    #[inline]
    fn queen_lower(mask: u64, empty: u64) -> u64 {
        const MASK: [u64; 4] =
            [!BoardFile::A.mask(), !0, !BoardFile::H.mask(), !BoardFile::H.mask()];
        let empty = Qu64::from_array([empty; 4]) & Qu64::from_array(MASK);
        let mut prop = Qu64::from_array([mask; 4]);
        let mut fill = Qu64::from_array([mask; 4]);

        for _ in 0..5 {
            prop = prop >> Qu64::from_array(SHIFT) & empty;
            fill |= prop;
        }

        fill |= (prop >> Qu64::from_array(SHIFT)) & empty;

        (fill >> Qu64::from_array(SHIFT) & Qu64::from_array(MASK)).reduce_or()
    }
}

pub fn king_fill(mask: u64) -> u64 {
    return king_fill_upper(mask) | king_fill_lower(mask);

    const SHIFT: [u64; 4] = [
        Direction::NorthWest as u64,
        Direction::North as u64,
        Direction::NorthEast as u64,
        Direction::East as u64,
    ];

    #[inline]
    pub fn king_fill_upper(mask: u64) -> u64 {
        const MASK: [u64; 4] =
            [!BoardFile::A.mask(), !0, !BoardFile::H.mask(), !BoardFile::H.mask()];

        ((Qu64::from_array([mask; 4]) & Qu64::from_array(MASK)) << Qu64::from_array(SHIFT))
            .reduce_or()
    }

    #[inline]
    pub fn king_fill_lower(mask: u64) -> u64 {
        const MASK: [u64; 4] =
            [!BoardFile::H.mask(), !0, !BoardFile::A.mask(), !BoardFile::A.mask()];
        ((Qu64::from_array([mask; 4]) & Qu64::from_array(MASK)) >> Qu64::from_array(SHIFT))
            .reduce_or()
    }
}

pub fn knight_fill(mask: u64) -> u64 {
    return knight_fill_upper(mask) | knight_fill_lower(mask);

    const SHIFT: [u64; 4] = [
        (Direction::North as i8 + Direction::NorthEast as i8) as u64,
        (Direction::North as i8 + Direction::NorthWest as i8) as u64,
        (Direction::East as i8 + Direction::NorthEast as i8) as u64,
        (Direction::West as i8 + Direction::NorthWest as i8) as u64,
    ];

    #[inline]
    pub fn knight_fill_upper(mask: u64) -> u64 {
        const MASK: [u64; 4] = [
            !BoardFile::H.mask(),
            !BoardFile::A.mask(),
            !BoardFile::G.mask() & !BoardFile::H.mask(),
            !BoardFile::A.mask() & !BoardFile::B.mask(),
        ];

        ((Qu64::from_array([mask; 4]) & Qu64::from_array(MASK)) << Qu64::from_array(SHIFT))
            .reduce_or()
    }

    #[inline]
    pub fn knight_fill_lower(mask: u64) -> u64 {
        const MASK: [u64; 4] = [
            !BoardFile::A.mask(),
            !BoardFile::H.mask(),
            !BoardFile::A.mask() & !BoardFile::B.mask(),
            !BoardFile::G.mask() & !BoardFile::H.mask(),
        ];

        ((Qu64::from_array([mask; 4]) & Qu64::from_array(MASK)) >> Qu64::from_array(SHIFT))
            .reduce_or()
    }
}

#[inline]
pub fn white_pawn_move_fill(mask: u64, empty: u64) -> u64 {
    let start = mask & BoardRank::_2.mask();
    (mask << 8 & empty) | ((start << 8 & empty) << 8 & empty)
}

#[inline]
pub fn black_pawn_move_fill(mask: u64, empty: u64) -> u64 {
    let start = mask & BoardRank::_7.mask();
    mask >> 8 & empty | (start >> 8 & empty) >> 8 & empty
}

#[inline]
pub fn white_pawn_attack_fill(mask: u64) -> u64 {
    mask << 9 & !BoardFile::A.mask() | mask << 7 & !BoardFile::H.mask()
}

#[inline]
pub fn black_pawn_attack_fill(mask: u64) -> u64 {
    mask >> 9 & !BoardFile::H.mask() | mask >> 7 & !BoardFile::A.mask()
}

#[test]
fn fill_test() {
    let mut rng = pi_rng();

    for _ in 0..100 {
        let total = rng.next_u64() & rng.next_u64();
        for sq1 in Squares(u64::MAX) {
            for sq2 in Squares(u64::MAX) {
                let pieces = sq1.bit() | sq2.bit();
                let total = total | pieces;

                assert_eq!(queen_fill(pieces, !total), queen_attacks(pieces, total));
                assert_eq!(rook_fill(pieces, !total), rook_attacks(pieces, total));
                assert_eq!(bishop_fill(pieces, !total), bishop_attacks(pieces, total));
                assert_eq!(knight_fill(pieces), knight_attacks(pieces));
                assert_eq!(king_fill(pieces), king_attacks(pieces));
            }
        }
    }
}
