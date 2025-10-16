use std::{
    simd::{Simd, num::SimdUint},
    time::Instant,
};

use rand::RngCore;

use crate::{
    bits::{
        Squares,
        attacks::{bishop_attacks, queen_attacks, rook_attacks},
    },
    fuzzing::test::pi_rng,
    model::{Dir, Square},
};

#[inline]
pub const fn shift_north(mask: u64) -> u64 {
    mask << 8
}

#[inline]
pub const fn shift_south(mask: u64) -> u64 {
    mask >> 8
}

#[inline]
pub const fn shift_east(mask: u64) -> u64 {
    mask << 1
}

#[inline]
pub const fn shift_west(mask: u64) -> u64 {
    mask & Square::WEST_EDGE >> 1
}

pub fn fill_rook4(rooks: u64, total: u64) -> u64 {
    let empty = Simd::from_array([!total, !total]);
    let mut rooks_ne = Simd::from_array([rooks, rooks]);
    let mut rooks_sw = Simd::from_array([rooks, rooks]);
    let mut flood_ne = Simd::from_array([rooks, rooks]);
    let mut flood_sw = Simd::from_array([rooks, rooks]);

    const SHIFT: Simd<u64, 2> = Simd::from_array([Dir::North as u64, Dir::East as u64]);
    const SHL_MASK: Simd<u64, 2> = Simd::from_array([Square::NORTH_EDGE, Square::EAST_EDGE]);
    const SHR_MASK: Simd<u64, 2> = Simd::from_array([Square::SOUTH_EDGE, Square::WEST_EDGE]);

    for _ in 0..5 {
        rooks_ne = ((rooks_ne & SHL_MASK) << SHIFT) & empty;
        rooks_sw = ((rooks_sw & SHR_MASK) >> SHIFT) & empty;

        flood_ne |= rooks_ne;
        flood_sw |= rooks_sw;
    }

    flood_ne |= ((rooks_ne & SHL_MASK) << SHIFT) & empty;
    flood_sw |= ((rooks_sw & SHR_MASK) >> SHIFT) & empty;

    flood_ne = (flood_ne & SHL_MASK) << SHIFT;
    flood_sw = (flood_sw & SHR_MASK) >> SHIFT;

    flood_ne.reduce_or() | flood_sw.reduce_or()
}

pub fn fill_bishop4(bishops: u64, total: u64) -> u64 {
    let empty = Simd::from_array([!total, !total]);

    let mut bishops_n = Simd::from_array([bishops, bishops]);
    let mut bishops_s = Simd::from_array([bishops, bishops]);

    let mut flood_n = Simd::from_array([bishops, bishops]);
    let mut flood_s = Simd::from_array([bishops, bishops]);

    const SHIFT: Simd<u64, 2> = Simd::from_array([Dir::NorthEast as u64, Dir::NorthWest as u64]);

    const SHL_MASK: Simd<u64, 2> = Simd::from_array([
        Square::NORTH_EDGE & Square::EAST_EDGE,
        Square::NORTH_EDGE & Square::WEST_EDGE,
    ]);

    const SHR_MASK: Simd<u64, 2> = Simd::from_array([
        Square::SOUTH_EDGE & Square::WEST_EDGE,
        Square::SOUTH_EDGE & Square::EAST_EDGE,
    ]);

    for _ in 0..5 {
        bishops_n = ((bishops_n & SHL_MASK) << SHIFT) & empty;
        bishops_s = ((bishops_s & SHR_MASK) >> SHIFT) & empty;

        flood_n |= bishops_n;
        flood_s |= bishops_s;
    }

    flood_n |= ((bishops_n & SHL_MASK) << SHIFT) & empty;
    flood_s |= ((bishops_s & SHR_MASK) >> SHIFT) & empty;

    flood_n = (flood_n & SHL_MASK) << SHIFT;
    flood_s = (flood_s & SHR_MASK) >> SHIFT;

    flood_n.reduce_or() | flood_s.reduce_or()
}

#[inline]
pub fn fill_queen4(queens: u64, total: u64) -> u64 {
    // rewrite to be rook + bishop directly
    let empty = Simd::from_array([!total, !total, !total, !total]);

    let mut queens_u = Simd::from_array([queens, queens, queens, queens]);
    let mut queens_d = Simd::from_array([queens, queens, queens, queens]);

    let mut flood_u = Simd::from_array([queens, queens, queens, queens]);
    let mut flood_d = Simd::from_array([queens, queens, queens, queens]);

    const SHIFT: Simd<u64, 4> = Simd::from_array([
        Dir::NorthWest as u64,
        Dir::North as u64,
        Dir::NorthEast as u64,
        Dir::East as u64,
    ]);

    const SHL_MASK: Simd<u64, 4> = Simd::from_array([
        Square::NORTH_EDGE & Square::WEST_EDGE,
        Square::NORTH_EDGE,
        Square::NORTH_EDGE & Square::EAST_EDGE,
        Square::EAST_EDGE,
    ]);

    const SHR_MASK: Simd<u64, 4> = Simd::from_array([
        Square::SOUTH_EDGE & Square::EAST_EDGE,
        Square::SOUTH_EDGE,
        Square::SOUTH_EDGE & Square::WEST_EDGE,
        Square::WEST_EDGE,
    ]);

    for _ in 0..5 {
        queens_u = ((queens_u & SHL_MASK) << SHIFT) & empty;
        queens_d = ((queens_d & SHR_MASK) >> SHIFT) & empty;

        flood_u |= queens_u;
        flood_d |= queens_d;
    }

    flood_u |= ((queens_u & SHL_MASK) << SHIFT) & empty;
    flood_d |= ((queens_d & SHR_MASK) >> SHIFT) & empty;

    flood_u = (flood_u & SHL_MASK) << SHIFT;
    flood_d = (flood_d & SHR_MASK) >> SHIFT;

    flood_u.reduce_or() | flood_d.reduce_or()
}

#[test]
#[rustfmt::skip]
fn fill_test() {
    let mut rng = pi_rng();

    for _ in 0 .. 100 {
        let total = rng.next_u64() & rng.next_u64();
        for sq1 in Squares(u64::MAX) {
            for sq2 in Squares(u64::MAX) {
                let pieces = sq1.bit() | sq2.bit();
                let total = total | pieces;
                assert_eq!(fill_rook4(pieces, total), rook_attacks(pieces, total));
                assert_eq!(fill_bishop4(pieces, total), bishop_attacks(pieces, total));
                assert_eq!(fill_queen4(pieces, total), queen_attacks(pieces, total));
            }
        }
    }
}
