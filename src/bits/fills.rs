use std::{
    simd::{Simd, num::SimdUint},
    time::Instant,
};

use rand::RngCore;

use crate::{
    bits::{
        Bits,
        attacks::{bishop_attacks, queen_attacks, rook_attacks},
    },
    fuzzing::test::pi_rng,
    model::{Dir, Square},
};

#[inline]
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

#[inline]
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

#[inline]
pub fn fill_rook2(rooks: u64, total: u64) -> u64 {
    let empty = !total;

    let mut rooks_n = rooks;
    let mut rooks_e = rooks;
    let mut rooks_s = rooks;
    let mut rooks_w = rooks;

    let mut flood_n = rooks;
    let mut flood_e = rooks;
    let mut flood_s = rooks;
    let mut flood_w = rooks;

    for _ in 0..6 {
        rooks_n = ((rooks_n & 0x00FF_FFFF_FFFF_FFFF) << 8) & empty;
        rooks_e = ((rooks_e & 0x7F7F_7F7F_7F7F_7F7F) << 1) & empty;
        rooks_s = ((rooks_s & 0xFFFF_FFFF_FFFF_FF00) >> 8) & empty;
        rooks_w = ((rooks_w & 0xFEFE_FEFE_FEFE_FEFE) >> 1) & empty;

        flood_n |= rooks_n;
        flood_e |= rooks_e;
        flood_s |= rooks_s;
        flood_w |= rooks_w;
    }

    let res_n = (flood_n & 0x00FF_FFFF_FFFF_FFFF) << 8;
    let res_e = (flood_e & 0x7F7F_7F7F_7F7F_7F7F) << 1;
    let res_s = (flood_s & 0xFFFF_FFFF_FFFF_FF00) >> 8;
    let res_w = (flood_w & 0xFEFE_FEFE_FEFE_FEFE) >> 1;

    res_n | res_e | res_s | res_w
}

#[inline]
pub fn fill_bishop2(bishops: u64, total: u64) -> u64 {
    let empty = !total;

    let mut bishops_ne = bishops;
    let mut bishops_nw = bishops;
    let mut bishops_se = bishops;
    let mut bishops_sw = bishops;

    let mut flood_ne = bishops;
    let mut flood_nw = bishops;
    let mut flood_se = bishops;
    let mut flood_sw = bishops;

    for _ in 0..6 {
        bishops_ne = ((bishops_ne & 0x007F_7F7F_7F7F_7F7F) << 9) & empty;
        bishops_nw = ((bishops_nw & 0x00FE_FEFE_FEFE_FEFE) << 7) & empty;
        bishops_se = ((bishops_se & 0x7F7F_7F7F_7F7F_7F00) >> 7) & empty;
        bishops_sw = ((bishops_sw & 0xFEFE_FEFE_FEFE_FE00) >> 9) & empty;

        flood_ne |= bishops_ne;
        flood_nw |= bishops_nw;
        flood_se |= bishops_se;
        flood_sw |= bishops_sw;
    }

    let res_ne = (flood_ne & 0x007F_7F7F_7F7F_7F7F) << 9;
    let res_nw = (flood_nw & 0x00FE_FEFE_FEFE_FEFE) << 7;
    let res_se = (flood_se & 0x7F7F_7F7F_7F7F_7F00) >> 7;
    let res_sw = (flood_sw & 0xFEFE_FEFE_FEFE_FE00) >> 9;

    res_ne | res_nw | res_se | res_sw
}

#[inline]
pub fn fill_queen2(queens: u64, total: u64) -> u64 {
    let empty_n = !total & !0;
    let empty_e = !total & 0xFEFE_FEFE_FEFE_FEFE;
    let empty_s = !total & !0;
    let empty_w = !total & 0x7F7F_7F7F_7F7F_7F7F;
    let empty_ne = !total & 0xFEFE_FEFE_FEFE_FEFE;
    let empty_nw = !total & 0x7F7F_7F7F_7F7F_7F7F;
    let empty_se = !total & 0xFEFE_FEFE_FEFE_FEFE;
    let empty_sw = !total & 0x7F7F_7F7F_7F7F_7F7F;

    let mut queens_n = queens;
    let mut queens_e = queens;
    let mut queens_s = queens;
    let mut queens_w = queens;
    let mut queens_ne = queens;
    let mut queens_nw = queens;
    let mut queens_se = queens;
    let mut queens_sw = queens;

    let mut flood_n = queens;
    let mut flood_e = queens;
    let mut flood_s = queens;
    let mut flood_w = queens;
    let mut flood_ne = queens;
    let mut flood_nw = queens;
    let mut flood_se = queens;
    let mut flood_sw = queens;

    for _ in 0..6 {
        queens_n = (queens_n << 8) & empty_n;
        queens_e = (queens_e << 1) & empty_e;
        queens_ne = (queens_ne << 9) & empty_ne;
        queens_nw = (queens_nw << 7) & empty_nw;

        queens_s = (queens_s >> 8) & empty_s;
        queens_w = (queens_w >> 1) & empty_w;
        queens_se = (queens_se >> 7) & empty_se;
        queens_sw = (queens_sw >> 9) & empty_sw;

        flood_n |= queens_n;
        flood_e |= queens_e;
        flood_s |= queens_s;
        flood_w |= queens_w;
        flood_ne |= queens_ne;
        flood_nw |= queens_nw;
        flood_se |= queens_se;
        flood_sw |= queens_sw;
    }

    let res_n = flood_n << 8 & !0;
    let res_e = flood_e << 1 & 0xFEFE_FEFE_FEFE_FEFE;
    let res_ne = flood_ne << 9 & 0xFEFE_FEFE_FEFE_FEFE;
    let res_nw = flood_nw << 7 & 0x7F7F_7F7F_7F7F_7F7F;

    let res_w = flood_w >> 1 & 0x7F7F_7F7F_7F7F_7F7F;
    let res_s = flood_s >> 8 & !0;
    let res_se = flood_se >> 7 & 0xFEFE_FEFE_FEFE_FEFE;
    let res_sw = flood_sw >> 9 & 0x7F7F_7F7F_7F7F_7F7F;

    res_n | res_e | res_s | res_w | res_ne | res_nw | res_se | res_sw
}

/////////////////////////////////////////////////////

#[inline]
pub fn fill_rook_n(mut rooks: u64, total: u64) -> u64 {
    let empty = !total & !0;
    let mut flood = rooks;
    for _ in 0..6 {
        rooks = (rooks << 8) & empty;
        flood |= rooks;
    }
    flood << 8 & !0
}

#[inline]
pub fn fill_rook_e(mut rooks: u64, total: u64) -> u64 {
    let empty = !total & 0xFEFE_FEFE_FEFE_FEFE;
    let mut flood = rooks;
    for _ in 0..6 {
        rooks = (rooks << 1) & empty;
        flood |= rooks;
    }
    flood << 1 & 0xFEFE_FEFE_FEFE_FEFE
}

#[inline]
pub fn fill_rook_s(mut rooks: u64, total: u64) -> u64 {
    let empty = !total & !0;
    let mut flood = rooks;
    for _ in 0..6 {
        rooks = (rooks >> 8) & empty;
        flood |= rooks;
    }
    flood >> 8 & !0
}

#[inline]
pub fn fill_rook_w(mut rooks: u64, total: u64) -> u64 {
    let empty = !total & 0x7F7F_7F7F_7F7F_7F7F;
    let mut flood = rooks;
    for _ in 0..6 {
        rooks = (rooks >> 1) & empty;
        flood |= rooks;
    }
    flood >> 1 & 0x7F7F_7F7F_7F7F_7F7F
}

#[inline]
pub fn fill_rook(mut rooks: u64, total: u64) -> u64 {
    fill_rook_n(rooks, total)
        | fill_rook_s(rooks, total)
        | fill_rook_e(rooks, total)
        | fill_rook_w(rooks, total)
}

#[inline]
pub fn fill_bishop_ne(mut bishop: u64, total: u64) -> u64 {
    let empty = !total & 0xFEFE_FEFE_FEFE_FEFE;
    let mut flood = bishop;
    for _ in 0..6 {
        bishop = (bishop << 9) & empty;
        flood |= bishop;
    }
    flood << 9 & 0xFEFE_FEFE_FEFE_FEFE
}

#[inline]
pub fn fill_bishop_nw(mut bishops: u64, total: u64) -> u64 {
    let empty = !total & 0x7F7F_7F7F_7F7F_7F7F;
    let mut flood = bishops;
    for _ in 0..6 {
        bishops = (bishops << 7) & empty;
        flood |= bishops;
    }
    flood << 7 & 0x7F7F_7F7F_7F7F_7F7F
}

#[inline]
pub fn fill_bishop_se(mut bishop: u64, total: u64) -> u64 {
    let empty = !total & 0xFEFE_FEFE_FEFE_FEFE;
    let mut flood = bishop;
    for _ in 0..6 {
        bishop = (bishop >> 7) & empty;
        flood |= bishop;
    }
    flood >> 7 & 0xFEFE_FEFE_FEFE_FEFE
}

#[inline]
pub fn fill_bishop_sw(mut bishops: u64, total: u64) -> u64 {
    let empty = !total & 0x7F7F_7F7F_7F7F_7F7F;
    let mut flood = bishops;
    for _ in 0..6 {
        bishops = (bishops >> 9) & empty;
        flood |= bishops;
    }
    flood >> 9 & 0x7F7F_7F7F_7F7F_7F7F
}

#[inline]
pub fn fill_bishop(bishops: u64, total: u64) -> u64 {
    fill_bishop_ne(bishops, total)
        | fill_bishop_se(bishops, total)
        | fill_bishop_nw(bishops, total)
        | fill_bishop_sw(bishops, total)
}

#[inline]
pub fn fill_queen(queens: u64, total: u64) -> u64 {
    fill_rook_n(queens, total)
        | fill_rook_s(queens, total)
        | fill_rook_e(queens, total)
        | fill_rook_w(queens, total)
        | fill_bishop_ne(queens, total)
        | fill_bishop_se(queens, total)
        | fill_bishop_nw(queens, total)
        | fill_bishop_sw(queens, total)
}

#[test]
#[rustfmt::skip]
fn fill_test() {
    let mut rng = pi_rng();

    for _ in 0 .. 100 {
        let total = rng.next_u64() & rng.next_u64();
        for sq1 in Bits(u64::MAX) {
            for sq2 in Bits(u64::MAX) {
                let pieces = sq1.bit() | sq2.bit();
                let total = total | pieces;
                assert_eq!(fill_rook(pieces, total), rook_attacks(pieces, total));
                assert_eq!(fill_rook2(pieces, total), rook_attacks(pieces, total));
                assert_eq!(fill_rook4(pieces, total), rook_attacks(pieces, total));
                assert_eq!(fill_bishop(pieces, total), bishop_attacks(pieces, total));
                assert_eq!(fill_bishop2(pieces, total), bishop_attacks(pieces, total));
                assert_eq!(fill_bishop4(pieces, total), bishop_attacks(pieces, total));
                assert_eq!(fill_queen(pieces, total), queen_attacks(pieces, total));
                assert_eq!(fill_queen2(pieces, total), queen_attacks(pieces, total));
                assert_eq!(fill_queen4(pieces, total), queen_attacks(pieces, total));
            }
        }
    }
}
