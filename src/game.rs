use std::{collections::HashMap, sync::LazyLock};

use crate::{
    bits::board::{self, BitBoard},
    model::{Victory, moves::ChessMove},
    notation::{
        algebraic::AlgebraicMove,
        pgn::{PGN, PGNTags},
    },
    profile::Profile,
    zobrist::{ZOBHASHER, ZobHash},
};

pub struct GameState {
    pub board: BitBoard,
    pub outcome: Option<Victory>,
    pub hash: ZobHash,
    pub halfmoves: u16,
    pub moves: Vec<ChessMove>,
    pub seen_positions: HashMap<ZobHash, u8>,
    pub move_sequence: Vec<FatMove>,
    pub white: Option<Profile>,
    pub black: Option<Profile>,
    pub pgn_tags: PGNTags,
}

impl GameState {
    fn startpos() -> Self {
        Self::from_position(BitBoard::startpos(), 0)
    }

    fn from_position(board: BitBoard, halfmoves: u16) -> Self {
        let mut moves = vec![];
        board.moves(&mut moves);
        let hash = ZOBHASHER.hash(&board);

        Self {
            board,
            moves,
            hash,
            halfmoves: 0,
            seen_positions: hash_map! { hash => 1 },
            pgn_tags: PGNTags::default(),
            move_sequence: vec![],
            outcome: None,
            white: None,
            black: None,
        }
    }

    fn to_pgn(&self) -> PGN {}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FatMove {
    precon: ZobHash,
    chessmove: ChessMove,
    algebraic: AlgebraicMove,
}
