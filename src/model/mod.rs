pub mod castling;
pub mod metadata;
pub mod moves;
pub mod square;
pub mod tests;
pub mod wincon;

use std::num::NonZeroI8;

use crate::arrays::ArrayBoard;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct Square(NonZeroI8);

impl Square {
    pub const fn new(ix: i8) -> Option<Self> {
        match ix {
            0..=63 => {
                if let Some(nzi8) = NonZeroI8::new(ix + 1) {
                    return Some(Self(nzi8));
                } else {
                    return None;
                }
            }
            _ => None,
        }
    }

    pub const fn ix(self) -> i8 {
        self.0.get() - 1
    }

    pub const fn at(f: File, r: Rank) -> Self {
        Self::new(f as i8 + r as i8).unwrap()
    }

    pub const fn file_rank(self) -> (File, Rank) {
        (
            File::new(self.ix() & 0x7).unwrap(),
            Rank::new(self.ix() >> 3).unwrap(),
        )
    }

    pub const fn next(self) -> Option<Self> {
        if self.0.get() == 64 {
            None
        } else {
            Some(Self(unsafe { NonZeroI8::new_unchecked(self.0.get() + 1) }))
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i8)]
pub enum File {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7,
}

impl File {
    pub const fn by(self, r: Rank) -> Square {
        Square::at(self, r)
    }

    pub const fn ix(self) -> i8 {
        self as i8
    }

    pub const fn new(ix: i8) -> Option<Self> {
        match ix {
            0..=7 => Some(unsafe { std::mem::transmute(ix) }),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i8)]
pub enum Rank {
    _1 = 0,
    _2 = 8,
    _3 = 16,
    _4 = 24,
    _5 = 32,
    _6 = 40,
    _7 = 48,
    _8 = 56,
}

impl Rank {
    pub const fn ix(self) -> i8 {
        self as i8
    }

    pub const fn new(ix: i8) -> Option<Self> {
        match ix {
            0..=7 => Some(unsafe { std::mem::transmute(ix * 8) }),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i8)]
pub enum Color {
    White = 1,
    Black = -1,
}

impl Color {
    pub const fn piece(self, p: Piece) -> ColorPiece {
        ColorPiece::new(self, p)
    }

    pub const fn opposite(self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }

    pub const fn rank(self) -> Rank {
        match self {
            Color::White => Rank::_1,
            Color::Black => Rank::_8,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i8)]
pub enum Piece {
    Pawn = 1,
    Knight = 2,
    Bishop = 3,
    Rook = 4,
    Queen = 5,
    King = 6,
}

impl Piece {
    pub const fn color(self, c: Color) -> ColorPiece {
        ColorPiece::new(c, self)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i8)]
pub enum ColorPiece {
    WhitePawn = 1,
    WhiteKnight = 2,
    WhiteBishop = 3,
    WhiteRook = 4,
    WhiteQueen = 5,
    WhiteKing = 6,
    BlackPawn = -1,
    BlackKnight = -2,
    BlackBishop = -3,
    BlackRook = -4,
    BlackQueen = -5,
    BlackKing = -6,
}

impl ColorPiece {
    pub const fn piece(self) -> Piece {
        use ColorPiece::*;
        use Piece::*;
        match self {
            WhitePawn | BlackPawn => Pawn,
            WhiteKnight | BlackKnight => Knight,
            WhiteBishop | BlackBishop => Bishop,
            WhiteRook | BlackRook => Rook,
            WhiteQueen | BlackQueen => Queen,
            WhiteKing | BlackKing => King,
        }
    }

    pub const fn color(self) -> Color {
        use Color::*;
        use ColorPiece::*;
        match self {
            WhitePawn | WhiteKnight | WhiteBishop | WhiteRook | WhiteQueen | WhiteKing => White,
            BlackPawn | BlackKnight | BlackBishop | BlackRook | BlackQueen | BlackKing => Black,
        }
    }

    pub const fn new(c: Color, p: Piece) -> Self {
        use Color::*;
        use ColorPiece::*;
        use Piece::*;
        match (c, p) {
            (White, Pawn) => WhitePawn,
            (White, Knight) => WhiteKnight,
            (White, Bishop) => WhiteBishop,
            (White, Rook) => WhiteRook,
            (White, Queen) => WhiteQueen,
            (White, King) => WhiteKing,
            (Black, Pawn) => BlackPawn,
            (Black, Knight) => BlackKnight,
            (Black, Bishop) => BlackBishop,
            (Black, Rook) => BlackRook,
            (Black, Queen) => BlackQueen,
            (Black, King) => BlackKing,
        }
    }

    pub const fn split(self) -> (Color, Piece) {
        use Color::*;
        use ColorPiece::*;
        use Piece::*;
        match self {
            WhitePawn => (White, Pawn),
            WhiteKnight => (White, Knight),
            WhiteBishop => (White, Bishop),
            WhiteRook => (White, Rook),
            WhiteQueen => (White, Queen),
            WhiteKing => (White, King),
            BlackPawn => (Black, Pawn),
            BlackKnight => (Black, Knight),
            BlackBishop => (Black, Bishop),
            BlackRook => (Black, Rook),
            BlackQueen => (Black, Queen),
            BlackKing => (Black, King),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i8)]
pub enum Dir {
    North = 8,
    East = 1,
    South = -8,
    West = -1,
    NorthEast = 9,
    SouthEast = -7,
    SouthWest = -9,
    NorthWest = 7,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i8)]
pub enum Victory {
    WhiteWins = 1,
    BlackWins = 2,
    Draw = 3,
}
