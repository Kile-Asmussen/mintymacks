use core::time;
use std::{array, fmt::Write, iter::Inspect};

use crate::{bits::board::BitBoard, notation::{fen::{parse_fen, render_fen}, uci::{find_literal_uci, literal_uci, next_uci_token, parse_many_uci, parse_uci, split_at_uci, Line, Uci}}, print_uci};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UciGui {
    Uci(),
    Debug(bool),
    IsReady(),
    SetOption(String, OptVal),
    UciNewGame(),
    Go(GoCommand),
    PonderHit(),
    Position(PositionString, Line),
    Quit(),
    Register(Registration),
    Stop(),
}

impl UciGui {
    pub fn to_string(&self) -> String {
        let mut res = vec![];
        self.print(&mut res);
        res.join(" ")
    }

    pub fn from_str(s: &str) -> Option<Self> {
        let input = s.split_whitespace().collect::<Vec<_>>();
        let (res, _) = parse_uci(&input[..])?;
        Some(res)
    }
}

impl Uci for UciGui {
    fn print(&self, output: &mut Vec<String>) {
        match self {
            Self::Uci() => print_uci!(output, "uci"),
            Self::Debug(b) => print_uci!(output, "debug", (if *b { "on" } else { "off" }) ),
            Self::IsReady() => print_uci!(output, "isready"),
            Self::SetOption(name, value) => print_uci!(output, "setoption", "name", name, value),
            Self::UciNewGame() => print_uci!(output, "ucinewgame"),
            Self::Go(go_command) => print_uci!(output, "go", go_command),
            Self::PonderHit() => print_uci!(output, "ponderhit"),
            Self::Position(position_string, moves) if moves.is_empty() => print_uci!(output, "position", position_string),
            Self::Position(position_string, moves) => print_uci!(output, "position", position_string, "moves", &moves[..]),
            Self::Quit() => print_uci!(output, "quit"),
            Self::Register(registration) => print_uci!(output, "register", registration),
            Self::Stop() => print_uci!(output, "stop"),
        }
    }

    fn parse_direct<'a>(input: &'a [&'a str]) -> Option<(Self, &'a [&'a str])> {
        if let Some(input) = literal_uci("uci", input) {
            return Some((Self::Uci(), input));
        }

        if let Some(input) = literal_uci("isready", input) {
            return Some((Self::IsReady(), input));
        }

        if let Some(input) = literal_uci("ucinewgame", input) {
            return Some((Self::UciNewGame(), input));
        }

        if let Some(input) = literal_uci("ponderhit", input) {
            return Some((Self::PonderHit(), input));
        }

        if let Some(input) = literal_uci("quit", input) {
            return Some((Self::Quit(), input));
        }

        if let Some(input) = literal_uci("stop", input) {
            return Some((Self::Stop(), input));
        }

        if let Some(input) = literal_uci("debug", input) {
            if let Some(input) = find_literal_uci("on", input) {
                return Some((Self::Debug(true), input));
            }

            if let Some(input) = find_literal_uci("off", input) {
                return Some((Self::Debug(false), input));
            }

            return None;
        }

        if let Some(input) = literal_uci("setoption", input)
        && let Some(input) = literal_uci("name", input)
        && let Some((name, input)) = next_uci_token(input)
        && let Some((optval, input)) = parse_uci(input) {
            return Some((Self::SetOption(name, optval), input));
        }

        if let Some(input) = literal_uci("register", input)
        && let Some((registration, input)) = parse_uci(input) {
            return Some((Self::Register(registration), input));
        }

        if let Some(input) = literal_uci("go", input)
        && let Some((go, input)) = parse_uci(input) {
            return Some((Self::Go(go), input));
        }

        if let Some(input) = literal_uci("position", input)
        && let Some((position, input)) = parse_uci(input) {

            if let Some(input) = find_literal_uci("moves", input)
            && let Some((moves, input)) = parse_many_uci(input) {
                return Some((Self::Position(position, moves), input));    
            }

            return Some((Self::Position(position, vec![]), input));
        }


        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OptVal {
    Button(),
    Check(bool),
    StringOrCombo(String),
    Spin(u64)
}

impl Uci for OptVal {
    fn print(&self, output: &mut Vec<String>) {
        match self {
            Self::Button() => {}
            Self::Check(b) => print_uci!(output, "value", b),
            Self::StringOrCombo(s) => print_uci!(output, "value", s),
            Self::Spin(n) => print_uci!(output, "value", n),
        }
    }

    fn parse_direct<'a>(input: &'a [&'a str]) -> Option<(Self, &'a [&'a str])> {
        if let Some(input) = literal_uci("value", input) {
            if let Some((b, input)) = parse_uci(input) {
                return Some((Self::Check(b), input));
            } else if let Some((n, input)) = parse_uci(input) {
                return Some((Self::Spin(n), input))
            } else {
                return Some((Self::StringOrCombo(input.join(" ")), &[]))
            }
        } else {
            return Some((Self::Button(), input));
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GoCommand {
    SearchMoves(Line),
    Ponder(),
    Time(TimeControl),
    Depth(u64),
    Nodes(u64), 
    Mate(u64),
    Movetime(u64),
    Infinite(),
    Perft(Option<u64>)
}

impl Uci for GoCommand {
    fn print(&self, output: &mut Vec<String>) {
        match self {
            Self::SearchMoves(moves) => print_uci!(output, "searchmoves", &moves[..]),
            Self::Ponder() => print_uci!(output, "ponder"),
            Self::Time(time_control) => print_uci!(output, time_control),
            Self::Depth(n) => print_uci!(output, "depth", n),
            Self::Nodes(n) => print_uci!(output, "nodes", n),
            Self::Mate(n) => print_uci!(output, "mate", n),
            Self::Movetime(n) => print_uci!(output, "movetime", n),
            Self::Infinite() => print_uci!(output, "infinite"),
            Self::Perft(n) => print_uci!(output, "perft", n)
        }
    }

    fn parse_direct<'a>(input: &'a [&'a str]) -> Option<(Self, &'a [&'a str])> {
        if let Some(input) = literal_uci("ponder", input) {
            return Some((Self::Ponder(), input));
        }

        if let Some(input) = literal_uci("infinite", input) {
            return Some((Self::Infinite(), input));
        }

        if let Some(input) = literal_uci("depth", input)
        && let Some((n, input)) = parse_uci(input) {
            return Some((Self::Depth(n), input));
        }

        if let Some(input) = literal_uci("nodes", input)
        && let Some((n, input)) = parse_uci(input) {
            return Some((Self::Nodes(n), input));
        }

        if let Some(input) = literal_uci("mate", input)
        && let Some((n, input)) = parse_uci(input) {
            return Some((Self::Mate(n), input));
        }

        if let Some(input) = literal_uci("movetime", input)
        && let Some((n, input)) = parse_uci(input) {
            return Some((Self::Movetime(n), input));
        }

        if let Some(input) = literal_uci("perft", input) {
            if let Some((n, input)) = parse_uci(input) {
                return Some((Self::Perft(Some(n)), input));
            } else {   
                return Some((Self::Perft(None), input));
            }
        }

        if let Some(input) = literal_uci("searchmoves", input)
        && let Some((moves, input)) = parse_many_uci(input) {
            return Some((Self::SearchMoves(moves), input));
        }

        if let Some((tc, input)) = parse_uci(input) {
            return Some((Self::Time(tc), input))
        }

        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimeControl {
    pub wtime: u64,
    pub btime: u64,
    pub winc: u64,
    pub binc: u64,
    pub moves_to_go: u64
}

impl Uci for TimeControl {
    fn print(&self, output: &mut Vec<String>) {
        if self.wtime != 0 {
            print_uci!(output, "wtime", self.wtime);
        }

        if self.btime != 0 {
            print_uci!(output, "btime", self.btime);
        }

        if self.winc != 0 {
            print_uci!(output, "winc", self.winc);
        }

        if self.binc != 0 {
            print_uci!(output, "binc", self.binc);
        }

        if self.moves_to_go != 0 {
            print_uci!(output, "movestogo", self.moves_to_go);
        }
    }

    fn parse_direct<'a>(input: &'a [&'a str]) -> Option<(Self, &'a [&'a str])> {

        if !input.iter().any(|s| ["wtime", "btime"].contains(s)) {
            return None;
        }

        let (wtime, input1) = find_literal_uci("wtime", input).and_then(|i| parse_uci(i)).unwrap_or((0, input));

        let (btime, input2) = find_literal_uci("btime", input).and_then(|i| parse_uci(i)).unwrap_or((0, input));

        let (winc, input3) = find_literal_uci("winc", input).and_then(|i| parse_uci(i)).unwrap_or((0, input));

        let (binc, input4) = find_literal_uci("binc", input).and_then(|i| parse_uci(i)).unwrap_or((0, input));

        let (moves_to_go, input5) = find_literal_uci("movestogo", input).and_then(|i| parse_uci(i)).unwrap_or((0, input));

        let inputs = [input1, input2, input3, input4, input5];
        let input = inputs.iter().min_by_key(|s| s.len())?;

        Some((Self { wtime, btime, winc, binc, moves_to_go }, input))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PositionString {
    Fen([String; 6]),
    Startpos()
}

impl Uci for PositionString {
    fn print(&self, output: &mut Vec<String>) {
        match self {
            Self::Fen(fen) => print_uci!(output, "fen", &fen[..]),
            Self::Startpos() => print_uci!(output, "startpos"),
        }
    }

    fn parse_direct<'a>(input: &'a [&'a str]) -> Option<(Self, &'a [&'a str])> {
        if let Some(input) = literal_uci("startpos", input) {
            return Some((Self::Startpos(), input));
        }

        if let Some(input) = literal_uci("fen", input)
        && let Some((fen, input)) = split_at_uci("moves", input) {
            if let (&[x], _) = fen.as_chunks() {
                return Some((Self::Fen(x.map(|s| s.to_string())), input))
            } else {
                return None;
            }
        }

        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Registration {
    Later(),
    NameCode(String, String),
}

impl Uci for Registration {
    fn print(&self, output: &mut Vec<String>) {
        match self {
            Self::Later() => print_uci!(output, "later"),
            Self::NameCode(name, code) => print_uci!(output, "name", name, "code", code),
        }
    }

    fn parse_direct<'a>(input: &'a [&'a str]) -> Option<(Self, &'a [&'a str])> {
        if let Some(input) = literal_uci("later", input) {
            return Some((Self::Later(), input));
        }

        if let Some(input) = literal_uci("name", input)
        && let Some((name, input)) = split_at_uci("code", input)
        && let Some(input) = find_literal_uci("code", input) {
            return Some((Self::NameCode(name.join(" "), input.join(" ")), input))
        }

        None
    }
}