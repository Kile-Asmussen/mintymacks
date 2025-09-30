use std::{collections::HashMap, hash::Hash, time::Duration};

use crate::notation::{algebraic::{self, AlgebraicMove}, regexp};

#[derive(Debug)]
pub struct PGN {
    pub event: String,
    pub site: String,
    pub date: String,
    pub round: String,
    pub white: String,
    pub black: String,
    pub result: String,
    pub annotator: Option<String>,
    pub ply_count: Option<String>,
    pub time_control: Option<String>,
    pub termination: Option<String>,
    pub mode: Option<String>,
    pub fen: Option<String>,
    pub tag_pairs: HashMap<String, String>,
    pub moves: Vec<MovePair>,
    pub end: String,
}

impl PGN {
    pub fn to_string(&self) -> String {
        let mut res = String::new();

        add_tag_pair(&mut res, "Event", Some(&self.event));
        add_tag_pair(&mut res, "Site", Some(&self.site));
        add_tag_pair(&mut res, "Date", Some(&self.date));
        add_tag_pair(&mut res, "Round", Some(&self.round));
        add_tag_pair(&mut res, "White", Some(&self.white));
        add_tag_pair(&mut res, "Black", Some(&self.black));
        add_tag_pair(&mut res, "Result", Some(&self.result));

        add_tag_pair(&mut res, "Annotator", self.annotator.as_deref());
        add_tag_pair(&mut res, "PlyCount", self.ply_count.as_deref());
        add_tag_pair(&mut res, "TimeControl", self.time_control.as_deref());
        add_tag_pair(&mut res, "Termination", self.termination.as_deref());
        add_tag_pair(&mut res, "Mode", self.mode.as_deref());
        add_tag_pair(&mut res, "Fen", self.fen.as_deref());

        for (k, v) in &self.tag_pairs {
            add_tag_pair(&mut res, k, Some(v));
        }
        
        res += "\n";

        for mv in &self.moves {
            res += &mv.to_string();
        }

        if !self.end.is_empty() {
            res.push(' ');
            res += &self.end;
            res += "\n\n"
        }

        return res;

        fn add_tag_pair(res: &mut String, name: &str, value: Option<&str>) {
            if let Some(value) = value {
                *res += &format!(r#"[{name} "{value}"]"#);
                *res += "\n";
            }
        }
    }

    pub fn parse(file: &str) -> (Option<Self>, &str) {
        let (mut tag_pairs, file) = Self::parse_tag_pairs(file);

        let (moves, file) = Self::parse_game(file);

        let (end, file) = Self::parse_end(file);

        (
            try {
                PGN {
                    event: tag_pairs.remove("Event")?,
                    site: tag_pairs.remove("Site")?,
                    date: tag_pairs.remove("Date")?,
                    round: tag_pairs.remove("Round")?,
                    white: tag_pairs.remove("White")?,
                    black: tag_pairs.remove("Black")?,
                    result: tag_pairs.remove("Result")?,
                    annotator: tag_pairs.remove("Annotator"),
                    ply_count: tag_pairs.remove("PlyCount"),
                    time_control: tag_pairs.remove("TimeControl"),
                    termination: tag_pairs.remove("Termination"),
                    mode: tag_pairs.remove("Mode"),
                    fen: tag_pairs.remove("FEN"),
                    tag_pairs,
                    moves,
                    end,
                }
            },
            file
        )
    }

    pub fn parse_tag_pairs(mut file: &str) -> (HashMap<String, String>, &str) {
        let mut res = hash_map!{};

        while let Some(c) = regexp!(r#"\s*\[\s*(\w+)\s+"([^"]*)"\s*\]"#).captures(file) {
            let (matched, [tag, value]) = c.extract::<2>();
            res.insert(tag.to_string(), value.to_string());
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

    pub fn parse_end(file: &str) -> (String, &str) {
        if let Some(c) = regexp!(r"\s+(1-0|0-1|1/2-1/2|)").captures(file) {
            let (full, [cap]) = c.extract::<1>();
            (cap.to_string(), &file[full.len()..])
        } else {
            ("".to_string(), file)
        }
    }
}

#[derive(Clone, Debug)]
pub struct MovePair {
    pub turn: u64,
    pub white: Option<AlgebraicMove>,
    pub white_nag: u64,
    pub white_comment: Option<String>,
    pub black: Option<AlgebraicMove>,
    pub black_nag: u64,
    pub black_comment: Option<String>,
}

impl MovePair {
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
        let mut white : Option<Option<AlgebraicMove>> = None;
        let mut white_nag = 0;
        let mut white_comment : Option<String> = None;
        let mut black : Option<AlgebraicMove> = None;
        let mut black_nag = 0;
        let mut black_comment : Option<String> = None;

        loop {
            if tokens.is_empty() {
                break;
            }

            match &tokens[0] {
                GameToken::TurnCounter(n) if turn == 0 => turn = *n,
                GameToken::TurnCounter(n) if turn == *n => {},
                GameToken::DotDot() if white.is_none() => white = Some(None),
                GameToken::DotDot() if white.is_some() => {}
                GameToken::NumAnGlyph(n) if white.is_some() => white_nag = *n,
                GameToken::NumAnGlyph(n) if black.is_some() => black_nag = *n,
                GameToken::NumAnGlyph(_) => {}
                GameToken::Move(alg) if white.is_none() => white = Some(Some(alg.clone())),
                GameToken::Move(alg) if white.is_some() && black.is_none() => black = Some(alg.clone()),
                GameToken::Comment(com) if white.is_some() && black.is_none() && white_comment.is_none() => white_comment = Some(com.clone()),
                GameToken::Comment(com) if white.is_some() && black.is_some() && black_comment.is_none() => black_comment = Some(com.clone()),
                GameToken::Comment(_) if white_comment.is_some() && black_comment.is_some() => {}
                _ => break
            }

            tokens = &tokens[1..];
        }

        if let Some(white) = white && turn != 0 && (white.is_some() || black.is_some()) {
            (Some(
                MovePair { turn, white, white_nag, white_comment, black, black_nag, black_comment }
            ), tokens)
        } else {
            (None, tokens)
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GameToken {
    TurnCounter(u64),
    NumAnGlyph(u64),
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
                res.push(GameToken::TurnCounter(u64::from_str_radix(num, 10).unwrap()));
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
                res.push(GameToken::NumAnGlyph(u64::from_str_radix(num, 10).unwrap()));
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