use std::{collections::HashMap, sync::LazyLock};

use crate::{
    bits::board::{self, BitBoard},
    model::{Color, Victory, moves::ChessMove},
    notation::{
        algebraic::AlgebraicMove,
        fen::render_fen,
        pgn::{MovePair, PGN, PGNTags, Tag},
    },
    profile::Profile,
    zobrist::{ZOBHASHER, ZobHash},
};

pub struct GameState {
    pub start: Option<(BitBoard, u16)>,
    pub board: BitBoard,
    pub outcome: Option<Victory>,
    pub hash: ZobHash,
    pub halfmoves: u16,
    pub moves: Vec<ChessMove>,
    pub seen_positions: HashMap<ZobHash, u8>,
    pub move_sequence: Vec<FatMove>,
    pub white: Option<Profile>,
    pub black: Option<Profile>,
}

impl GameState {
    pub fn startpos() -> Self {
        Self::from_position_internal(BitBoard::startpos(), 0, false)
    }

    pub fn from_position(board: BitBoard, halfmoves: u16) -> Self {
        Self::from_position_internal(board, halfmoves, true)
    }

    fn from_position_internal(board: BitBoard, halfmoves: u16, store: bool) -> Self {
        let mut moves = vec![];
        board.moves(&mut moves);
        let hash = ZOBHASHER.hash(&board);

        Self {
            board: board.clone(),
            moves,
            hash,
            start: if store {
                Some((board, halfmoves))
            } else {
                None
            },
            halfmoves: 0,
            seen_positions: hash_map! { hash => 1 },
            move_sequence: vec![],
            outcome: None,
            white: None,
            black: None,
        }
    }

    pub fn build_pgn_header(&self) -> PGNTags {
        let mut res = PGNTags::default();
        let today = chrono::Utc::now().date_naive();
        res.canon
            .insert(Tag::Date, today.format("%Y.%m.%d").to_string().into());
        res.canon.insert(
            Tag::Result,
            self.outcome.map(|v| v.to_ascii()).unwrap_or("*").into(),
        );
        if let Some(w) = &self.white {
            res.canon.insert(Tag::White, w.name().to_string().into());
        }
        if let Some(b) = &self.black {
            res.canon.insert(Tag::Black, b.name().to_string().into());
        }
        if let Some((board, halfmove)) = &self.start {
            res.canon
                .insert(Tag::FEN, render_fen(board, *halfmove).into());
        }

        res
    }

    pub fn to_pgn(&self) -> PGN {
        let (color, turn) = self
            .start
            .as_ref()
            .map(|(b, _)| (b.metadata.to_move, b.metadata.turn))
            .unwrap_or((Color::White, 1));

        PGN {
            headers: self.build_pgn_header(),
            moves: MovePair::pair_moves(
                self.move_sequence.iter().map(|f| f.algebraic),
                turn,
                color == Color::Black,
            ),
            end: self.outcome,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FatMove {
    precon: ZobHash,
    chessmove: ChessMove,
    algebraic: AlgebraicMove,
}
