use std::{arch::x86_64, array, ops::BitXor};

use rand::{Rng, SeedableRng};

use crate::{
    arrays::ArrayBoard,
    bits::{
        Bits, Mask, bit,
        board::{BitBoard, HalfBitBoard},
    },
    model::{Color, castling::CastlingRights, moves::Move},
};

type Hash = u64;

const BLACK_TO_MOVE: Hash = 1 << 63;
const HASH_BITS: Hash = !BLACK_TO_MOVE;

pub fn zob<R: Rng>(rng: &mut R) -> Hash {
    rng.next_u64() & HASH_BITS
}

impl ArrayBoard<Hash> {
    pub fn hash(&self, m: Mask) -> Hash {
        Bits(m).map(|sq| self.at(sq)).fold(0, u64::bitxor)
    }

    pub fn new_from_rng<R: Rng>(rng: &mut R) -> Self {
        ArrayBoard(array::from_fn(|_| zob(rng)))
    }
}

#[derive(Debug, Clone)]
pub struct ZobristHalfBoard {
    pub pawns: ArrayBoard<Hash>,
    pub knights: ArrayBoard<Hash>,
    pub bishops: ArrayBoard<Hash>,
    pub rooks: ArrayBoard<Hash>,
    pub queens: ArrayBoard<Hash>,
    pub kings: ArrayBoard<Hash>,
}

impl ZobristHalfBoard {
    pub fn new_from_rng<R: Rng>(rng: &mut R) -> ZobristHalfBoard {
        ZobristHalfBoard {
            pawns: ArrayBoard::new_from_rng(rng),
            knights: ArrayBoard::new_from_rng(rng),
            bishops: ArrayBoard::new_from_rng(rng),
            rooks: ArrayBoard::new_from_rng(rng),
            queens: ArrayBoard::new_from_rng(rng),
            kings: ArrayBoard::new_from_rng(rng),
        }
    }

    pub fn hash(&self, hboard: &HalfBitBoard) -> Hash {
        self.pawns.hash(hboard.pawns)
            ^ self.knights.hash(hboard.knights)
            ^ self.bishops.hash(hboard.bishops)
            ^ self.rooks.hash(hboard.rooks)
            ^ self.queens.hash(hboard.queens)
            ^ self.kings.hash(hboard.kings)
    }
}

#[derive(Debug, Clone)]
pub struct ZobristCastling {
    pub white_eastward: Hash,
    pub white_westward: Hash,
    pub black_eastward: Hash,
    pub black_westward: Hash,
}

impl ZobristCastling {
    pub fn new_from_rng<R: Rng>(rng: &mut R) -> Self {
        ZobristCastling {
            white_eastward: zob(rng),
            white_westward: zob(rng),
            black_eastward: zob(rng),
            black_westward: zob(rng),
        }
    }

    pub fn hash(&self, cr: CastlingRights) -> Hash {
        let mut res = Hash::MIN;

        if cr.eastward(Color::White) {
            res ^= self.white_eastward;
        }

        if cr.westward(Color::White) {
            res ^= self.white_westward;
        }

        if cr.eastward(Color::Black) {
            res ^= self.black_eastward;
        }

        if cr.westward(Color::Black) {
            res ^= self.black_westward;
        }

        res
    }
}

#[derive(Debug, Clone)]
pub struct ZobristBoard {
    pub white: ZobristHalfBoard,
    pub black: ZobristHalfBoard,
    pub en_passant: ArrayBoard<Hash>,
    pub castling: ZobristCastling,
}

impl ZobristBoard {
    pub fn new() -> ZobristBoard {
        Self::new_from_rng(&mut rand::rngs::StdRng::from_seed(
            *b"3.141592653589793238462643383279",
        ))
    }

    pub fn new_from_rng<R: Rng>(rng: &mut R) -> ZobristBoard {
        ZobristBoard {
            white: ZobristHalfBoard::new_from_rng(rng),
            black: ZobristHalfBoard::new_from_rng(rng),
            en_passant: ArrayBoard::new_from_rng(rng),
            castling: ZobristCastling::new_from_rng(rng),
        }
    }

    pub fn hash(&self, board: &BitBoard) -> Hash {
        self.white.hash(&board.white)
            ^ self.black.hash(&board.black)
            ^ self.en_passant.hash(bit(board.en_passant))
            ^ self.castling.hash(board.castling_rights)
            ^ if board.to_move == Color::Black {
                BLACK_TO_MOVE
            } else {
                Hash::MIN
            }
    }

    pub fn delta(&self, h: Hash, mv: Move) -> Hash {
        todo!()
    }
}
