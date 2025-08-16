use crate::{
    bits::Mask,
    board::{ColorPiece, Square},
};

pub struct ArrayBoard<T: Copy>(pub [T; 64]);

impl<T: Copy> ArrayBoard<T> {
    pub const fn new(t: T) -> Self {
        Self([t; 64])
    }

    pub const fn at(&self, sq: Square) -> T {
        self.0[sq.ix() as usize]
    }

    pub const fn set(&mut self, sq: Square, t: T) {
        self.0[sq.ix() as usize] = t
    }

    pub const fn setup(board: [[T; 8]; 8]) -> Self {
        let [
            [a8, b8, c8, d8, e8, f8, g8, h8],
            [a7, b7, c7, d7, e7, f7, g7, h7],
            [a6, b6, c6, d6, e6, f6, g6, h6],
            [a5, b5, c5, d5, e5, f5, g5, h5],
            [a4, b4, c4, d4, e4, f4, g4, h4],
            [a3, b3, c3, d3, e3, f3, g3, h3],
            [a2, b2, c2, d2, e2, f2, g2, h2],
            [a1, b1, c1, d1, e1, f1, g1, h1],
        ] = board;
        Self([
            a1, b1, c1, d1, e1, f1, g1, h1, //
            a2, b2, c2, d2, e2, f2, g2, h2, //
            a3, b3, c3, d3, e3, f3, g3, h3, //
            a4, b4, c4, d4, e4, f4, g4, h4, //
            a5, b5, c5, d5, e5, f5, g5, h5, //
            a6, b6, c6, d6, e6, f6, g6, h6, //
            a7, b7, c7, d7, e7, f7, g7, h7, //
            a8, b8, c8, d8, e8, f8, g8, h8, //
        ])
    }

    pub const fn iter<'a>(&'a self) -> ByteBoardIter<'a, T> {
        ByteBoardIter(Some(Square::a1), self)
    }
}

impl<'a, T: Copy> IntoIterator for &'a ArrayBoard<T> {
    type Item = (Square, T);

    type IntoIter = ByteBoardIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct ByteBoardIter<'a, T: Copy>(Option<Square>, &'a ArrayBoard<T>);

impl<'a, T: Copy> ByteBoardIter<'a, T> {
    pub const fn next(&mut self) -> Option<(Square, T)> {
        if let Some(sq) = self.0 {
            self.0 = sq.next();
            Some((sq, self.1.at(sq)))
        } else {
            None
        }
    }
}

impl<'a, T: Copy> Iterator for ByteBoardIter<'a, T> {
    type Item = (Square, T);

    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}

impl ArrayBoard<bool> {
    pub const fn mask(&self) -> Mask {
        let mut it = self.iter();
        let mut res = Mask::MIN;
        while let Some((sq, b)) = it.next() {
            if b {
                res |= 1 << sq.ix();
            }
        }
        res
    }
}

impl ArrayBoard<Option<ColorPiece>> {
    pub const fn mask(&self, p: ColorPiece) -> Mask {
        let mut it = self.iter();
        let mut res = Mask::MIN;
        while let Some((sq, op)) = it.next() {
            if let Some(p2) = op {
                if p as i8 == p2 as i8 {
                    res |= 1 << sq.ix();
                }
            }
        }
        res
    }
}
