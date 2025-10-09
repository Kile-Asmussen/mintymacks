use std::{collections::HashMap, sync::LazyLock};

use crate::{
    bits::board::{self, BitBoard},
    model::moves::ChessMove,
    notation::{
        algebraic::AlgebraicMove,
        pgn::{PGN, PGNHeaders},
    },
    profile::Profile,
    zobrist::{ZOBHASHER, ZobHash},
};

pub struct GameState {
    pub board: BitBoard,
    pub hash: ZobHash,
    pub halfmoves: u16,
    pub moves: Vec<ChessMove>,
    pub seen_positions: HashMap<ZobHash, u8>,
    pub move_sequence: Vec<FatMove>,
    pub white: Option<Profile>,
    pub black: Option<Profile>,
    pub pgn: PGN,
}

impl GameState {
    fn startpos() -> Self {
        let board = BitBoard::startpos();
        let mut moves = vec![];
        board.moves(&mut moves);
        let hash = ZOBHASHER.hash(&board);

        Self {
            board,
            moves,
            hash,
            halfmoves: 0,
            seen_positions: hash_map! { hash => 1 },
            pgn: PGN::new(),
            move_sequence: vec![],
            white: None,
            black: None,
        }
    }
}

pub struct FatMove {
    precon: ZobHash,
    chessmove: ChessMove,
    algebraic: AlgebraicMove,
}
