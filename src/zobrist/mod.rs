mod tests;

use std::{arch::x86_64, array, fs::canonicalize, ops::BitXor};

use rand::{Rng, SeedableRng};

use crate::{
    arrays::ArrayBoard,
    bits::{
        Bits, Mask, bit,
        board::{BitBoard, HalfBitBoard},
    },
    model::{
        Color, Piece, Square,
        castling::{CastlingDetail, CastlingDetails, CastlingRights},
        metadata::Metadata,
        moves::{Move, PseudoMove, Special},
    },
};

pub type ZobHash = u64;

pub fn zob<R: Rng>(rng: &mut R) -> ZobHash {
    rng.next_u64()
}

impl ArrayBoard<ZobHash> {
    pub fn hash(&self, m: Mask) -> ZobHash {
        Bits(m).map(|sq| self.at(sq)).fold(0, u64::bitxor)
    }

    pub fn at2(&self, pm: PseudoMove) -> ZobHash {
        self.at(pm.from) ^ self.at(pm.to)
    }

    pub fn new_from_rng<R: Rng>(rng: &mut R) -> Self {
        ArrayBoard(array::from_fn(|_| zob(rng)))
    }
}

#[derive(Debug, Clone)]
pub struct ZobristHalfBoard {
    pub pawns: ArrayBoard<ZobHash>,
    pub knights: ArrayBoard<ZobHash>,
    pub bishops: ArrayBoard<ZobHash>,
    pub rooks: ArrayBoard<ZobHash>,
    pub queens: ArrayBoard<ZobHash>,
    pub kings: ArrayBoard<ZobHash>,
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

    pub fn hash(&self, hboard: &HalfBitBoard) -> ZobHash {
        self.pawns.hash(hboard.pawns)
            ^ self.knights.hash(hboard.knights)
            ^ self.bishops.hash(hboard.bishops)
            ^ self.rooks.hash(hboard.rooks)
            ^ self.queens.hash(hboard.queens)
            ^ self.kings.hash(hboard.kings)
    }

    pub fn piece(&self, piece: Piece) -> &ArrayBoard<ZobHash> {
        match piece {
            Piece::Pawn => &self.pawns,
            Piece::Knight => &self.knights,
            Piece::Bishop => &self.bishops,
            Piece::Rook => &self.rooks,
            Piece::Queen => &self.queens,
            Piece::King => &self.kings,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ZobristCastling {
    pub white_eastward: ZobHash,
    pub white_westward: ZobHash,
    pub black_eastward: ZobHash,
    pub black_westward: ZobHash,
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

    pub fn hash(&self, cr: CastlingRights) -> ZobHash {
        let mut res = ZobHash::MIN;

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
    pub metadata: ZobristMetadata,
}

impl ZobristBoard {
    pub fn new() -> ZobristBoard {
        Self::new_from_rng(&mut rand::rngs::SmallRng::from_seed(
            *b"3.141592653589793238462643383279",
        ))
    }

    pub fn new_from_rng<R: Rng>(rng: &mut R) -> ZobristBoard {
        ZobristBoard {
            white: ZobristHalfBoard::new_from_rng(rng),
            black: ZobristHalfBoard::new_from_rng(rng),
            metadata: ZobristMetadata::new_from_rng(rng),
        }
    }

    pub fn hash(&self, board: &BitBoard) -> ZobHash {
        self.white.hash(&board.white)
            ^ self.black.hash(&board.black)
            ^ self.metadata.hash(board.metadata)
    }

    pub fn active_passive(&self, color: Color) -> (&ZobristHalfBoard, &ZobristHalfBoard) {
        match color {
            Color::White => (&self.white, &self.black),
            Color::Black => (&self.black, &self.white),
        }
    }

    pub fn delta(&self, mv: Move, details: CastlingDetails) -> ZobHash {
        let (act, pas) = self.active_passive(mv.piece.color());

        let movement = match mv.special {
            Some(Special::Promotion(p)) => act.pawns.at(mv.mv.from) ^ act.piece(p).at(mv.mv.to),
            Some(Special::CastlingEastward) => {
                let cast = details.eastward.reify(mv.piece.color());
                act.kings.at2(cast.king_move) ^ act.rooks.at2(cast.rook_move)
            }
            Some(Special::CastlingWestward) => {
                let cast = details.westward.reify(mv.piece.color());
                act.kings.at2(cast.king_move) ^ act.rooks.at2(cast.rook_move)
            }
            Some(Special::Null) => ZobHash::MIN,
            None => act.piece(mv.piece.piece()).at2(mv.mv),
        };

        let capture = if let Some((p, sq)) = mv.cap {
            pas.piece(p).hash(sq.bit())
        } else {
            ZobHash::MIN
        };

        let meta = self.metadata.hash_color(mv.piece.color())
            ^ self.metadata.hash_color(mv.piece.color().opposite())
            ^ self.metadata.hash_epc(mv.ep_opening())
            ^ self.metadata.hash_epc(mv.epc)
            ^ self.metadata.castling.hash(mv.rights)
            ^ self.metadata.castling.hash(mv.castling_change(details));

        movement ^ capture ^ meta
    }
}

#[derive(Debug, Clone)]
pub struct ZobristMetadata {
    pub en_passant: [ZobHash; 8],
    pub castling: ZobristCastling,
    pub black_to_move: ZobHash,
}

impl ZobristMetadata {
    pub fn new_from_rng<R: Rng>(rng: &mut R) -> ZobristMetadata {
        ZobristMetadata {
            en_passant: array::from_fn(|_| zob(rng)),
            castling: ZobristCastling::new_from_rng(rng),
            black_to_move: zob(rng),
        }
    }

    pub fn hash_color(&self, color: Color) -> ZobHash {
        if color == Color::Black {
            self.black_to_move
        } else {
            ZobHash::MIN
        }
    }

    pub fn hash_epc(&self, epc: Option<Square>) -> ZobHash {
        if let Some(sq) = epc {
            self.en_passant[sq.file_rank().0.ix() as usize]
        } else {
            ZobHash::MIN
        }
    }

    pub fn hash(&self, metadata: Metadata) -> ZobHash {
        self.castling.hash(metadata.castling_rights)
            ^ self.hash_color(metadata.to_move)
            ^ self.hash_epc(metadata.en_passant)
    }
}
