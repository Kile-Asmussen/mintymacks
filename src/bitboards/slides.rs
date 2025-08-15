use crate::{
    bitboards::{Mask, bit, mask},
    board::{Square, Squares},
    byteboard::ByteBoard,
};

pub const WEST: ByteBoard<Mask> = build_slideboard(-1, &Squares::WEST);
pub const EAST: ByteBoard<Mask> = build_slideboard(1, &Squares::EAST);
pub const NORTH: ByteBoard<Mask> = build_slideboard(8, &Squares::NORTH);
pub const SOUTH: ByteBoard<Mask> = build_slideboard(-8, &Squares::SOUTH);

pub const NORTHEAST: ByteBoard<Mask> = build_slideboard(7, &Squares::NORTHWEST);
pub const NORTHWEST: ByteBoard<Mask> = build_slideboard(9, &Squares::NORTHEAST);
pub const SOUTHEAST: ByteBoard<Mask> = build_slideboard(-7, &Squares::NORTHWEST);
pub const SOUTHWEST: ByteBoard<Mask> = build_slideboard(-9, &Squares::NORTHEAST);

pub const fn build_slideboard(dir: i8, max: &ByteBoard<i8>) -> ByteBoard<Mask> {
    let mut res = ByteBoard::new(0);
    let mut it = Squares::all();

    while let Some(sq) = it.next() {
        res.set(sq, build_slidemask(dir, max.at(sq), sq));
    }

    res
}

pub const fn build_slidemask(dir: i8, max: i8, sq: Square) -> Mask {
    let mut res = Mask::MIN;
    let mut n = 1;
    while n < max {
        let Some(s) = Square::new(sq.ix() + dir * n) else {
            break;
        };
        res |= bit(s);
        n += 1;
    }
    res
}

#[test]
fn slidemask_correct() {
    assert_eq!(
        build_slidemask(-1, 3, Square::d4),
        mask([
            0b_00000000,
            0b_00000000,
            0b_00000000,
            0b_00000000,
            0b_11100000, // 4
            0b_00000000,
            0b_00000000,
            0b_00000000,
            // abcd
        ])
    );

    assert_eq!(
        EAST.at(Square::a1),
        mask([
            0b_00000000,
            0b_00000000,
            0b_00000000,
            0b_00000000,
            0b_00000000,
            0b_00000000,
            0b_00000000,
            0b_01111111,
        ])
    )
}
