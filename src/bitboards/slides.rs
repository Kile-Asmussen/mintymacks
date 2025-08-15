use crate::{
    bitboards::{Mask, bit, mask, show_mask},
    board::{Square, Squares},
    byteboard::ArrayBoard,
};

pub const WEST: ArrayBoard<Mask> = build_slideboard(-1, &Squares::WEST);
pub const EAST: ArrayBoard<Mask> = build_slideboard(1, &Squares::EAST);
pub const NORTH: ArrayBoard<Mask> = build_slideboard(8, &Squares::NORTH);
pub const SOUTH: ArrayBoard<Mask> = build_slideboard(-8, &Squares::SOUTH);

pub const NORTHEAST: ArrayBoard<Mask> = build_slideboard(7, &Squares::NORTHWEST);
pub const NORTHWEST: ArrayBoard<Mask> = build_slideboard(9, &Squares::NORTHEAST);
pub const SOUTHEAST: ArrayBoard<Mask> = build_slideboard(-7, &Squares::NORTHWEST);
pub const SOUTHWEST: ArrayBoard<Mask> = build_slideboard(-9, &Squares::NORTHEAST);

pub const fn build_slideboard(dir: i8, max: &ArrayBoard<i8>) -> ArrayBoard<Mask> {
    let mut res = ArrayBoard::new(0);
    let mut it = Squares::all();

    while let Some(sq) = it.next() {
        res.set(sq, build_slidemask(dir, max.at(sq), sq));
    }

    res
}

pub const fn build_slidemask(dir: i8, max: i8, sq: Square) -> Mask {
    let mut res = Mask::MIN;
    let mut n = 1;
    while let Some(s) = Square::new(sq.ix() + dir * n)
        && n <= max
    {
        res |= bit(s);
        n += 1;
    }
    res
}

#[test]
fn slidemask_correct() {
    assert_eq!(
        EAST.at(Square::d4),
        mask([
            0b_00000000,
            0b_00000000,
            0b_00000000,
            0b_00000000,
            0b_00001111,
            0b_00000000,
            0b_00000000,
            0b_00000000,
        ])
    );

    assert_eq!(
        NORTH.at(Square::d4),
        mask([
            0b_00010000,
            0b_00010000,
            0b_00010000,
            0b_00010000,
            0b_00000000,
            0b_00000000,
            0b_00000000,
            0b_00000000,
        ])
    );

    assert_eq!(
        WEST.at(Square::d4),
        mask([
            0b_00000000,
            0b_00000000,
            0b_00000000,
            0b_00000000,
            0b_11100000,
            0b_00000000,
            0b_00000000,
            0b_00000000,
        ])
    );

    assert_eq!(
        SOUTH.at(Square::d4),
        mask([
            0b_00000000,
            0b_00000000,
            0b_00000000,
            0b_00000000,
            0b_00000000,
            0b_00010000,
            0b_00010000,
            0b_00010000,
        ])
    );
}
