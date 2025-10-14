use std::{
    collections::{HashMap, VecDeque},
    sync::LazyLock,
};

use crate::{
    bits::board::{self, BitBoard},
    deque,
    model::{Color, Victory, moves::ChessMove},
    notation::{
        MoveMatcher,
        algebraic::AlgebraicMove,
        fen::{parse_fen, render_fen, render_fen6},
        pgn::{MovePair, PGN, PGNTags},
        uci::{
            self, Line, Uci,
            gui::{PositionString, TimeControl, UciGui},
        },
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
    pub cursor: usize,
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
            move_sequence: vec![],
            cursor: 0,
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
            self.outcome.map(|v| v.to_str()).unwrap_or("*").into(),
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

        res
    }

    pub fn pgn_movelist(&self) -> Vec<MovePair> {
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

    pub fn uci_line(&self) -> Line {
        let mut res = Vec::with_capacity(self.move_sequence.len());
        for fm in &self.move_sequence {
            res.push(fm.longalg())
        }
        res
    }

    pub fn uci_position(&self) -> PositionString {
        if let Some(b) = &self.start {
            PositionString::Fen(render_fen6(b))
        } else {
            PositionString::Startpos()
        }
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
                        postcon: self.board.metadata.hash
                            ^ ZOBHASHER.delta(pm, self.board.metadata.castling_details),
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
                    postcon: self.board.metadata.hash
                        ^ ZOBHASHER.delta(pm, self.board.metadata.castling_details),
                });
            }
        }
        res
    }

    pub fn apply(&mut self, mut fm: FatMove) -> Option<FatMove> {
        if fm.apply(&mut self.board) {
            self.possible_moves.clear();
            self.board.moves(&mut self.possible_moves);
            *self
                .seen_positions
                .entry(self.board.pure_position_hash())
                .or_insert(0) += 1;
            self.outcome =
                Victory::determine(&self.board, &self.possible_moves, &self.seen_positions);

            if self.outcome
                == Some(Victory::from_color(
                    fm.chessmove.cpc.color(),
                    crate::model::WinReason::CheckMate,
                ))
                && fm.algebraic.check_or_mate == Some(false)
            {
                fm.algebraic.check_or_mate == Some(true);
            }

            self.move_sequence.push(fm);

            Some(fm)
        } else {
            None
        }
    }

    pub fn undo(&mut self) -> Option<FatMove> {
        if let Some(fm) = self.move_sequence.pop() {
            fm.unapply(&mut self.board);
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

    pub fn from_pgn(pgn: &PGN) -> Result<Self, String> {
        let mut res = GameState::startpos();

        if pgn.headers.0.get("SetUp").map(|s| &s[..]) == Some("1")
            && let Some(f) = pgn.headers.0.get("FEN")
        {
            let f = &f[..];
            res = GameState::from_position(parse_fen(f)?);
        }

        for mp in pgn.move_list() {
            res.apply(res.find_move(mp).map_err(|n| {
                if res.board.metadata.to_move == Color::White {
                    format!(
                        "Invalid PGN: {}. {} is not a valid and unambiguous move",
                        res.board.metadata.turn,
                        mp.to_string()
                    )
                } else {
                    format!(
                        "Invalid PGN: {}. .. {} is not a valid and unambiguous move",
                        res.board.metadata.turn,
                        mp.to_string()
                    )
                }
            })?);
        }

        res.outcome = pgn.end;

        Ok(res)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FatMove {
    pub precon: ZobHash,
    pub chessmove: ChessMove,
    pub algebraic: AlgebraicMove,
    pub postcon: ZobHash,
}

use crate::notation::LongAlg;

impl FatMove {
    pub fn longalg(&self) -> LongAlg {
        self.chessmove.simplify()
    }

    pub fn apply(self, board: &mut BitBoard) -> bool {
        if self.precon == board.metadata.hash {
            board.apply(self.chessmove);
            true
        } else {
            false
        }
    }

    pub fn unapply(self, board: &mut BitBoard) -> bool {
        if self.postcon == board.metadata.hash {
            board.unapply(self.chessmove);
            true
        } else {
            false
        }
    }
}

pub struct GameReview {
    pub tags: PGNTags,
    pub start: Option<Box<BitBoard>>,
    pub end: Box<BitBoard>,
    pub cursor: BitBoard,
    pub past: VecDeque<FatMove>,
    pub future: VecDeque<FatMove>,
}

impl GameReview {
    pub fn new(gs: &GameState, tags: PGNTags) -> Self {
        Self {
            tags,
            start: gs.start.clone(),
            end: Box::new(gs.board.clone()),
            cursor: gs
                .start
                .clone()
                .map(|b| *b)
                .unwrap_or_else(BitBoard::startpos),
            past: deque![],
            future: VecDeque::from(gs.move_sequence.clone()),
        }
    }

    pub fn to_start(&mut self) {
        if let Some(b) = &self.start {
            self.cursor = *b.clone()
        } else {
            self.cursor = BitBoard::startpos()
        }

        self.past.append(&mut self.future);
        self.future.append(&mut self.past);
    }

    pub fn next(&mut self) -> bool {
        if let Some(fm) = self.future.pop_front() {
            fm.apply(&mut self.cursor);
            self.past.push_back(fm);
            true
        } else {
            false
        }
    }

    pub fn prev(&mut self) -> bool {
        if let Some(fm) = self.past.pop_back() {
            fm.unapply(&mut self.cursor);
            self.future.push_front(fm);
            true
        } else {
            false
        }
    }

    pub fn to_end(&mut self) {
        self.cursor = (*self.end).clone();

        self.past.append(&mut self.future);
    }

    pub fn past_pgn(&self) -> Vec<MovePair> {
        MovePair::pair_moves(
            self.past.iter().map(|fm| fm.algebraic),
            self.start.as_ref().map(|b| b.metadata.turn).unwrap_or(0),
            self.start
                .as_ref()
                .map(|b| b.metadata.to_move == Color::Black)
                .unwrap_or(false),
        )
    }

    pub fn future_pgn(&self) -> Vec<MovePair> {
        MovePair::pair_moves(
            self.future.iter().map(|fm| fm.algebraic),
            self.cursor.metadata.turn,
            self.cursor.metadata.to_move == Color::Black,
        )
    }
}
