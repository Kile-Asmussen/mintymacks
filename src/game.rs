use std::{collections::HashMap, sync::LazyLock};

use crate::{
    bits::board::{self, BitBoard},
    model::{Color, Victory, moves::ChessMove},
    notation::{
        MoveMatcher,
        algebraic::AlgebraicMove,
        fen::render_fen,
        pgn::{MovePair, PGN, PGNTags},
        uci::{self, Uci, gui::TimeControl},
    },
    profile::Profile,
    zobrist::{ZOBHASHER, ZobHash},
};

pub struct GameState {
    pub start: Option<Box<BitBoard>>,
    pub board: BitBoard,
    pub outcome: Option<Victory>,
    pub possible_moves: Vec<ChessMove>,
    pub seen_positions: HashMap<ZobHash, u8>,
    pub move_sequence: Vec<FatMove>,
    pub time_control: uci::gui::TimeControl,
    pub white: Option<Profile>,
    pub black: Option<Profile>,
}

impl GameState {
    pub fn startpos() -> Self {
        Self::from_position_internal(BitBoard::startpos(), false)
    }

    pub fn from_position(board: BitBoard) -> Self {
        Self::from_position_internal(board, true)
    }

    fn from_position_internal(board: BitBoard, store: bool) -> Self {
        let mut moves = vec![];
        board.moves(&mut moves);

        Self {
            possible_moves: moves,
            start: if store {
                Some(Box::new(board.clone()))
            } else {
                None
            },
            seen_positions: hash_map! { board.metadata.hash => 1 },
            board: board,
            time_control: TimeControl::default(),
            move_sequence: vec![],
            outcome: None,
            white: None,
            black: None,
        }
    }

    pub fn pgn_header(&self) -> PGNTags {
        let mut res = PGNTags::default();
        let today = chrono::Utc::now().date_naive();
        res.0
            .insert("Date".into(), today.format("%Y.%m.%d").to_string().into());
        res.0.insert(
            "Result".into(),
            self.outcome.map(|v| v.to_ascii()).unwrap_or("*").into(),
        );
        if let Some(w) = &self.white {
            res.0.insert("White".into(), w.name().to_string().into());
        }
        if let Some(b) = &self.black {
            res.0.insert("Black".into(), b.name().to_string().into());
        }
        if let Some(board) = &self.start {
            res.0.insert("FEN".into(), render_fen(board).into());
            res.0.insert("SetUp".into(), "1".into());
        }
        if self.time_control != TimeControl::default() {
            res.0.insert("TimeControl".into(), {
                let mut v = vec![];
                self.time_control.print(&mut v);
                v.join(" ")
            });
        }
        res.0.insert("Mode".into(), "ICS".into());

        res
    }

    pub fn movelist(&self) -> Vec<MovePair> {
        let (color, turn) = self
            .start
            .as_ref()
            .map(|b| (b.metadata.to_move, b.metadata.turn))
            .unwrap_or((Color::White, 1));

        MovePair::pair_moves(
            self.move_sequence.iter().map(|f| f.algebraic),
            turn,
            color == Color::Black,
        )
    }

    pub fn find_move<M: MoveMatcher>(&self, m: M) -> Result<FatMove, usize> {
        let mut res = Err(0);
        for pm in &self.possible_moves {
            let pm = *pm;
            if m.matches(pm) {
                if let Err(0) = res {
                    res = Ok(FatMove {
                        chessmove: pm,
                        algebraic: pm.ambiguate(&self.board, &self.possible_moves),
                        precon: self.board.metadata.hash,
                    })
                } else if let Ok(_) = res {
                    res = Err(2)
                } else if let Err(n) = res {
                    res = Err(n + 1)
                }
            }
        }
        res
    }

    pub fn find_moves<M: MoveMatcher>(&self, m: M) -> Vec<FatMove> {
        let mut res = vec![];
        for pm in &self.possible_moves {
            let pm = *pm;
            if m.matches(pm) {
                res.push(FatMove {
                    chessmove: pm,
                    algebraic: pm.ambiguate(&self.board, &self.possible_moves),
                    precon: self.board.metadata.hash,
                });
            }
        }
        res
    }

    pub fn apply(&mut self, fm: FatMove) {
        if fm.precon == self.board.metadata.hash {
            self.move_sequence.push(fm);
            self.board.apply(fm.chessmove);
            self.possible_moves.clear();
            self.board.moves(&mut self.possible_moves);
            *self
                .seen_positions
                .entry(self.board.pure_position_hash())
                .or_insert(0) += 1;
            self.outcome =
                Victory::determine(&self.board, &self.possible_moves, &self.seen_positions)
        }
    }

    pub fn undo(&mut self) -> Option<FatMove> {
        if let Some(fm) = self.move_sequence.pop() {
            self.board.unapply(fm.chessmove);
            self.possible_moves.clear();
            self.board.moves(&mut self.possible_moves);
            *self
                .seen_positions
                .entry(self.board.pure_position_hash())
                .or_insert(1) -= 1;
            self.outcome =
                Victory::determine(&self.board, &self.possible_moves, &self.seen_positions);
            Some(fm)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FatMove {
    precon: ZobHash,
    chessmove: ChessMove,
    algebraic: AlgebraicMove,
}

use crate::notation::LongAlg;

impl FatMove {
    pub fn longalg(&self) -> LongAlg {
        self.chessmove.simplify()
    }
}
