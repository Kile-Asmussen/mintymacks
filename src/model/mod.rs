pub mod castling;
pub mod metadata;
pub mod moves;
pub mod square;
pub mod tests;
pub mod wincon;

use std::fmt::Debug;
use std::num::NonZeroI8;

use crate::arrays::ArrayBoard;
use crate::bits::{Bits, BoardMask};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

    pub const fn at(f: BoardFile, r: BoardRank) -> Self {
        Self(unsafe { NonZeroI8::new_unchecked(f as i8 + r as i8 + 1) })
    }

    pub const fn file_rank(self) -> (BoardFile, BoardRank) {
        (
            BoardFile::new(self.ix() & 0x7).unwrap(),
            BoardRank::new(self.ix() >> 3).unwrap(),
        )
    }

    pub const fn next(self) -> Option<Self> {
        if self.0.get() == 64 {
            None
        } else {
            Some(Self(unsafe { NonZeroI8::new_unchecked(self.0.get() + 1) }))
        }
    }

    pub const fn swap(self) -> Self {
        Self(unsafe { NonZeroI8::new_unchecked((0x38 ^ (self.0.get() - 1)) + 1) })
    }
}

impl Debug for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Square::")?;
        f.write_str(self.to_str())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
#[repr(i8)]
pub enum BoardFile {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7,
}

impl BoardFile {
    pub const fn by(self, r: BoardRank) -> Square {
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
#[repr(i8)]
pub enum BoardRank {
    _1 = 0,
    _2 = 8,
    _3 = 16,
    _4 = 24,
    _5 = 32,
    _6 = 40,
    _7 = 48,
    _8 = 56,
}

impl BoardRank {
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
#[repr(i8)]
pub enum Color {
    Black = -1,
    White = 1,
}

impl Color {
    pub const fn piece(self, p: ChessPiece) -> ColoredChessPiece {
        ColoredChessPiece::new(self, p)
    }

    pub const fn opposite(self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }

    pub const fn rank(self) -> BoardRank {
        match self {
            Color::White => BoardRank::_1,
            Color::Black => BoardRank::_8,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
#[repr(i8)]
pub enum ChessPiece {
    Pawn = 1,
    Knight = 2,
    Bishop = 3,
    Rook = 4,
    Queen = 5,
    King = 6,
}

impl ChessPiece {
    pub const fn color(self, c: Color) -> ColoredChessPiece {
        ColoredChessPiece::new(c, self)
    }

    pub const PAWN: i16 = 100;
    pub const KNIGHT: i16 = 325;
    pub const BISHOP: i16 = 333;
    pub const ROOK: i16 = 500;
    pub const QUEEN: i16 = 500;
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
#[repr(i8)]
pub enum ColoredChessPiece {
    BlackKing = -6,
    BlackQueen = -5,
    BlackRook = -4,
    BlackBishop = -3,
    BlackKnight = -2,
    BlackPawn = -1,
    WhitePawn = 1,
    WhiteKnight = 2,
    WhiteBishop = 3,
    WhiteRook = 4,
    WhiteQueen = 5,
    WhiteKing = 6,
}

impl ColoredChessPiece {
    pub const fn piece(self) -> ChessPiece {
        use ChessPiece::*;
        use ColoredChessPiece::*;
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
        use ColoredChessPiece::*;
        match self {
            WhitePawn | WhiteKnight | WhiteBishop | WhiteRook | WhiteQueen | WhiteKing => White,
            BlackPawn | BlackKnight | BlackBishop | BlackRook | BlackQueen | BlackKing => Black,
        }
    }

    pub const fn new(c: Color, p: ChessPiece) -> Self {
        use ChessPiece::*;
        use Color::*;
        use ColoredChessPiece::*;
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

    pub const fn split(self) -> (Color, ChessPiece) {
        use ChessPiece::*;
        use Color::*;
        use ColoredChessPiece::*;
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
#[repr(i8)]
pub enum Dir {
    SouthWest = -9,
    South = -8,
    SouthEast = -7,
    West = -1,
    East = 1,
    NorthWest = 7,
    North = 8,
    NorthEast = 9,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
#[repr(i8)]
pub enum Victory {
    BlackWins = -1,
    Draw = 0,
    WhiteWins = 1,
}
