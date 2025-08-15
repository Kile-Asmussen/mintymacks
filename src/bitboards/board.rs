use crate::bitboards::Mask;

pub struct HalfBitBoard {
    pub pawns: Mask,
    pub knights: Mask,
    pub bishops: Mask,
    pub rooks: Mask,
    pub queens: Mask,
    pub kings: Mask,
}

pub struct BitBoard {
    pub white: HalfBitBoard,
    pub black: HalfBitBoard,
}
