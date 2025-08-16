use crate::{
    bits::{Bits, Mask, bit, mask, slides},
    model::Square,
};

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

#[test]
fn slidemask_correct() {
    assert_eq!(
        slides::RAYS_EAST.at(Square::d4),
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
        slides::RAYS_NORTH.at(Square::d4),
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
        slides::RAYS_WEST.at(Square::d4),
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
        slides::RAYS_SOUTH.at(Square::d4),
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

    assert_eq!(
        slides::RAYS_NORTHEAST.at(Square::d4),
        mask([
            0b_00000001,
            0b_00000010,
            0b_00000100,
            0b_00001000,
            0b_00000000,
            0b_00000000,
            0b_00000000,
            0b_00000000,
        ])
    );

    assert_eq!(
        slides::RAYS_NORTHWEST.at(Square::d4),
        mask([
            0b_00000000,
            0b_10000000,
            0b_01000000,
            0b_00100000,
            0b_00000000,
            0b_00000000,
            0b_00000000,
            0b_00000000,
        ])
    );

    assert_eq!(
        slides::RAYS_SOUTHEAST.at(Square::d4),
        mask([
            0b_00000000,
            0b_00000000,
            0b_00000000,
            0b_00000000,
            0b_00000000,
            0b_00001000,
            0b_00000100,
            0b_00000010,
        ])
    );

    assert_eq!(
        slides::RAYS_SOUTHWEST.at(Square::d4),
        mask([
            0b_00000000,
            0b_00000000,
            0b_00000000,
            0b_00000000,
            0b_00000000,
            0b_00100000,
            0b_01000000,
            0b_10000000,
        ])
    );
}
