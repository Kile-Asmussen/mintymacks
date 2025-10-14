use std::{
    borrow::Cow,
    collections::HashMap,
    default,
    hash::Hash,
    ops::{Deref, Index},
    time::{Duration, Instant},
};

use strum::{EnumIter, IntoStaticStr, VariantArray};

use indexmap::IndexMap;

use crate::{
    ix_map,
    model::{DrawReason, Victory, WinReason},
    notation::{
        algebraic::{self, AlgebraicMove},
        regexp,
    },
};

pub fn load_pgn_file(mut file: &str) -> Vec<PGN> {
    let mut res = vec![];

    while let (Some(pgn), rest) = PGN::parse(file) {
        res.push(pgn);
        file = rest;
    }

    res
}

#[derive(Debug, Clone)]
pub struct PGNTags(pub IndexMap<Cow<'static, str>, String>);

impl Default for PGNTags {
    fn default() -> Self {
        Self(ix_map! {
            "Event".into() => "?".into(),
            "Site".into() => "?".into(),
            "Date".into() => "????.??.??".into(),
            "Round".into() => "?".into(),
            "White".into() => "?".into(),
            "Black".into() => "?".into(),
            "Result".into() => "*".into(),
        })
    }
}

impl PGNTags {
    pub fn from_tag_pairs(mut hash: IndexMap<Cow<'static, str>, String>) -> Self {
        let mut res = Self::default();
        for i in CANON_TAGS.iter().chain(SEMICANON_TAGS) {
            if let Some(v) = hash.shift_remove(&Cow::Borrowed(*i)) {
                res.0.insert((*i).into(), v);
            }
        }
        res
    }
}

pub const CANON_TAGS: &[&'static str] =
    &["Event", "Site", "Date", "Round", "White", "Black", "Result"];

pub const SEMICANON_TAGS: &[&'static str] = &[
    "Time",
    "TimeControl",
    "FEN",
    "SetUp",
    "ECO",
    "Opening",
    "Variation",
    "Mode",
    "PlyCount",
    "Termination",
    "Annotator",
];

#[derive(Debug, Clone)]
pub struct PGN {
    pub headers: PGNTags,
    pub moves: Vec<MovePair>,
    pub end: Option<Victory>,
}

impl PGN {
    pub fn new() -> Self {
        PGN {
            headers: PGNTags::default(),
            moves: vec![],
            end: None,
        }
    }

    pub fn to_string(&self, res: &mut String, newlines: bool) {
        for c in CANON_TAGS.iter().chain(SEMICANON_TAGS) {
            let Some(v) = self.headers.0.get(*c) else {
                continue;
            };
            *res += &format!("[{c} \"{v}\"]\n");
        }

        for (k, v) in &self.headers.0 {
            if CANON_TAGS.contains(&k.as_ref()) || SEMICANON_TAGS.contains(&k.as_ref()) {
                continue;
            }
            *res += &format!("[{k} \"{v}\"]\n");
        }

        *res += "\n";

        for mv in &self.moves {
            *res += &mv.to_string();
            res.push(if newlines { '\n' } else { ' ' });
        }

        *res += &self.end.map(|v| v.to_str()).unwrap_or("*");
        *res += "\n\n"
    }

    pub fn move_list(&self) -> Vec<AlgebraicMove> {
        self.moves
            .iter()
            .flat_map(|mp| mp.white.iter().chain(mp.black.iter()))
            .map(|mv| *mv)
            .collect()
    }

    pub fn parse(orig_file: &str) -> (Option<Self>, &str) {
        let (mut tag_pairs, file) = Self::parse_tag_pairs(orig_file);

        if tag_pairs.is_empty() {
            return (None, orig_file);
        }

        let (moves, file) = Self::parse_game(file);

        if moves.is_empty() {
            return (None, orig_file);
        }

        let (end, file) = Self::parse_end(file);

        let Some(end) = end else {
            return (None, orig_file);
        };

        let default = PGNTags::default();

        let res = PGN {
            headers: PGNTags::from_tag_pairs(tag_pairs),
            moves,
            end,
        };

        (Some(res), file)
    }

    pub fn parse_tag_pairs(mut file: &str) -> (IndexMap<Cow<'static, str>, String>, &str) {
        let mut res = ix_map! {};

        while let Some(c) = regexp!(r#"\A\s*\[\s*(\w+)\s+"([^"]*)"\s*\]"#).captures(file) {
            let (matched, [tag, value]) = c.extract::<2>();
            if let Some(tag) = CANON_TAGS.iter().chain(SEMICANON_TAGS).find(|t| **t == tag) {
                res.insert((*tag).into(), value.into());
            } else {
                res.insert(Cow::Owned(tag.into()), value.into());
            }
            file = &file[matched.len()..];
        }

        (res, file)
    }

    pub fn parse_game(file: &str) -> (Vec<MovePair>, &str) {
        let (all_tokens, file) = GameToken::parse_tokens(file);
        let mut tokens = &all_tokens[..];
        let mut res = vec![];

        while let (Some(mp), rest) = MovePair::parse_pair(tokens) {
            res.push(mp);
            tokens = rest;
        }

        (res, file)
    }

    pub fn parse_end(file: &str) -> (Option<Option<Victory>>, &str) {
        if let Some(c) = regexp!(r"\s+(\*|1-0|0-1|1/2-1/2)").captures(file) {
            let (full, [cap]) = c.extract::<1>();
            (
                Some(match cap {
                    "1-0" => Some(Victory::WhiteWins(WinReason::Unknown)),
                    "0-1" => Some(Victory::BlackWins(WinReason::Unknown)),
                    "1/2-1/2" => Some(Victory::Draw(DrawReason::Unknown)),
                    "*" => None,
                    _ => return (None, file),
                }),
                &file[full.len()..],
            )
        } else {
            (None, file)
        }
    }
}

#[derive(Clone, Debug)]
pub struct MovePair {
    pub turn: u16,
    pub white: Option<AlgebraicMove>,
    pub white_nag: u8,
    pub white_comment: Option<String>,
    pub black: Option<AlgebraicMove>,
    pub black_nag: u8,
    pub black_comment: Option<String>,
}

impl MovePair {
    pub fn pair_moves<I: IntoIterator<Item = AlgebraicMove>>(
        it: I,
        mut turn: u16,
        black_first: bool,
    ) -> Vec<Self> {
        let mut res = vec![];

        let mut it = it.into_iter();

        if black_first {
            res.push(MovePair {
                turn,
                white: None,
                white_nag: 0,
                white_comment: None,
                black: it.next(),
                black_nag: 0,
                black_comment: None,
            });
            turn += 1;
        }

        let mut it = it.array_chunks::<2>();

        while let Some([w, b]) = it.next() {
            res.push(MovePair {
                turn,
                white: Some(w),
                black: Some(b),
                white_comment: None,
                black_comment: None,
                white_nag: 0,
                black_nag: 0,
            });
            turn += 1;
        }

        if let Some(w) = it.into_remainder().and_then(|mut i| i.next()) {
            res.push(MovePair {
                turn,
                white: Some(w),
                white_nag: 0,
                white_comment: None,
                black: None,
                black_nag: 0,
                black_comment: None,
            });
        }

        res
    }

    pub fn to_string(&self) -> String {
        let mut res = String::new();

        res += &self.turn.to_string();
        res += ". ";

        if let Some(white) = self.white {
            res += &white.to_string();
        } else {
            res += "..";
        }

        if self.white_nag != 0 {
            res += &format!(" ${}", self.white_nag);
        }

        if let Some(ref white_comment) = self.white_comment {
            res += &format!(" {{{}}}", white_comment);
        }

        if let Some(black) = self.black {
            res.push(' ');
            res += &black.to_string();

            if self.black_nag != 0 {
                res += &format!(" ${}", self.black_nag);
            }

            if let Some(ref black_comment) = self.black_comment {
                res += &format!(" {{{}}}", black_comment);
            }
        }

        res
    }
}

impl PartialEq for MovePair {
    fn eq(&self, other: &Self) -> bool {
        self.turn == other.turn && self.white == other.white && self.black == other.black
    }
}

impl MovePair {
    pub fn parse_pair(mut tokens: &[GameToken]) -> (Option<MovePair>, &[GameToken]) {
        let mut turn = 0;
        let mut white: Option<Option<AlgebraicMove>> = None;
        let mut white_nag = 0;
        let mut white_comment: Option<String> = None;
        let mut black: Option<AlgebraicMove> = None;
        let mut black_nag = 0;
        let mut black_comment: Option<String> = None;

        loop {
            if tokens.is_empty() {
                break;
            }

            match &tokens[0] {
                GameToken::TurnCounter(n) if turn == 0 => turn = *n,
                GameToken::TurnCounter(n) if turn == *n => {}
                GameToken::DotDot() if white.is_none() => white = Some(None),
                GameToken::DotDot() if white.is_some() => {}
                GameToken::NumAnGlyph(n) if white.is_some() => white_nag = *n,
                GameToken::NumAnGlyph(n) if black.is_some() => black_nag = *n,
                GameToken::NumAnGlyph(_) => {}
                GameToken::Move(alg) if white.is_none() => white = Some(Some(alg.clone())),
                GameToken::Move(alg) if white.is_some() && black.is_none() => {
                    black = Some(alg.clone())
                }
                GameToken::Comment(com)
                    if white.is_some() && black.is_none() && white_comment.is_none() =>
                {
                    white_comment = Some(com.clone())
                }
                GameToken::Comment(com)
                    if white.is_some() && black.is_some() && black_comment.is_none() =>
                {
                    black_comment = Some(com.clone())
                }
                GameToken::Comment(_) if white_comment.is_some() && black_comment.is_some() => {}
                _ => break,
            }

            tokens = &tokens[1..];
        }

        if let Some(white) = white
            && turn != 0
            && (white.is_some() || black.is_some())
        {
            (
                Some(MovePair {
                    turn,
                    white,
                    white_nag,
                    white_comment,
                    black,
                    black_nag,
                    black_comment,
                }),
                tokens,
            )
        } else {
            (None, tokens)
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GameToken {
    TurnCounter(u16),
    NumAnGlyph(u8),
    DotDot(),
    Move(AlgebraicMove),
    Comment(String),
}

impl GameToken {
    pub fn parse_tokens(mut file: &str) -> (Vec<GameToken>, &str) {
        let mut res = vec![];
        loop {
            if let Some(c) = regexp!(r"^\s*(\d+)\.").captures(file) {
                let (full, [num]) = c.extract();
                res.push(GameToken::TurnCounter(
                    u16::from_str_radix(num, 10).unwrap(),
                ));
                file = &file[full.len()..];
            } else if let Some(c) = regexp!(r"^\s*\{([^}]*)\}").captures(file) {
                let (full, [comment]) = c.extract();
                res.push(GameToken::Comment(comment.to_string()));
                file = &file[full.len()..];
            } else if let Some(c) = regexp!(r"^\s*\.\.").find(file) {
                res.push(GameToken::DotDot());
                file = &file[c.len()..];
            } else if let Some(c) = regexp!(r"^\s*\$(\d+)").captures(file) {
                let (full, [num]) = c.extract();
                res.push(GameToken::NumAnGlyph(u8::from_str_radix(num, 10).unwrap()));
                file = &file[full.len()..];
            } else if let Some(c) = regexp!(r"^\s*(\S+)").captures(file) {
                let (full, [alg]) = c.extract();
                if let Some(algebraic) = AlgebraicMove::parse(alg) {
                    res.push(GameToken::Move(algebraic));
                    file = &file[full.len()..];
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        (res, file)
    }
}
