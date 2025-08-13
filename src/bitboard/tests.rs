use crate::{
    bitboard::{Bits, Mask, bit, mask},
    board::{Square, Squares},
};

#[test]
fn square_iter() {
    assert_eq!(
        Bits(Mask::MAX).collect::<Vec<_>>(),
        Squares.into_iter().collect::<Vec<_>>()
    );
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
        bit(Square::a1),
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
