use core::time;
use std::{fmt::Write, iter::Inspect};

use crate::{bits::board::BitBoard, notation::{fen::{parse_fen, render_fen}, uci::{literal_uci, parse_uci, token_uci, until_uci, Line, Uci}}, print_uci};

#[derive(Debug, Clone, PartialEq, Eq)]
enum UciGui {
    Uci(),
    Debug(bool),
    IsReady(),
    SetOption(String, Option<String>),
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
            Self::SetOption(name, None) => print_uci!(output, "setoption", "name", name),
            Self::SetOption(name, Some(value)) => print_uci!(output, "setoption", "name", name, "value", value),
            Self::UciNewGame() => print_uci!(output, "ucinewgame"),
            Self::Go(go_command) => print_uci!(output, "go", go_command),
            Self::PonderHit() => print_uci!(output, "ponderhit"),
            Self::Position(position_string, moves) if moves.is_empty() => print_uci!(output, "position", position_string),
            Self::Position(position_string, moves) => print_uci!(output, "position", position_string, "moves", moves),
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
            if let Some(input) = literal_uci("on", input) {
                return Some((Self::Debug(true), input));
            }

            if let Some(input) = literal_uci("off", input) {
                return Some((Self::Debug(true), input));
            }

            return None;
        }

        if let Some(input) = literal_uci("setoption", input)
        && let Some((name, input)) = token_uci(input) {
            if let Some(input) = literal_uci("value", input) {
                return Some((Self::SetOption(name, Some(input.join(" "))), &[]));
            }

            return Some((Self::SetOption(name, None), input));
        }

        if let Some(input) = literal_uci("stop", input)
        && let Some((registration, input)) = parse_uci(input) {
            return Some((Self::Register(registration), input));
        }

        if let Some(input) = literal_uci("position", input)
        && let Some((position, input)) = parse_uci(input) {

            if let Some(input) = literal_uci("moves", input)
            && let Some((moves, input)) = parse_uci(input) {
                return Some((Self::Position(position, moves), input));    
            }

            return Some((Self::Position(position, vec![]), input));
        }


        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum GoCommand {
    SearchMoves(Line),
    Ponder(),
    Time(TimeControl),
    Depth(u64),
    Nodes(u64), 
    Mate(u64),
    Movetime(u64),
    Infinite(),
}

impl Uci for GoCommand {
    fn print(&self, output: &mut Vec<String>) {
        match self {
            Self::SearchMoves(moves) => print_uci!(output, "searchmoves", moves),
            Self::Ponder() => print_uci!(output, "ponder"),
            Self::Time(time_control) => print_uci!(output, time_control),
            Self::Depth(n) => print_uci!(output, "depth", n),
            Self::Nodes(n) => print_uci!(output, "nodes", n),
            Self::Mate(n) => print_uci!(output, "mate", n),
            Self::Movetime(n) => print_uci!(output, "movetime", n),
            Self::Infinite() => print_uci!(output, "infinite"),
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

        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TimeControl {
    wtime: u64,
    btime: u64,
    winc: u64,
    binc: u64,
    moves_to_go: u64
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

        let (wtime, input1) = literal_uci("wtime", input).and_then(|i| parse_uci(input)).unwrap_or((0, input));

        let (btime, input2) = literal_uci("btime", input).and_then(|i| parse_uci(input)).unwrap_or((0, input));

        let (winc, input3) = literal_uci("winc", input).and_then(|i| parse_uci(input)).unwrap_or((0, input));

        let (binc, input4) = literal_uci("binc", input).and_then(|i| parse_uci(input)).unwrap_or((0, input));

        let (moves_to_go, input5) = literal_uci("moves_to_go", input).and_then(|i| parse_uci(input)).unwrap_or((0, input));

        let inputs = [input1, input2, input3, input4, input5];
        let input = inputs.iter().min_by_key(|s| s.len())?;

        Some((Self { wtime, btime, winc, binc, moves_to_go }, input))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum PositionString {
    Fen(BitBoard, u16),
    Startpos()
}

impl Uci for PositionString {
    fn print(&self, output: &mut Vec<String>) {
        match self {
            Self::Fen(fen, halfmove) => print_uci!(output, "fen", render_fen(fen, *halfmove)),
            Self::Startpos() => print_uci!(output, "startpos"),
        }
    }

    fn parse_direct<'a>(input: &'a [&'a str]) -> Option<(Self, &'a [&'a str])> {
        if let Some(input) = literal_uci("startpos", input) {
            return Some((Self::Startpos(), input));
        }

        if let Some(input) = literal_uci("fen", input)
        && let Some((fen, input)) = until_uci("moves", input)
        && let Some((fen, halfmove)) = parse_fen(&fen.join(" ")).ok() {
            return Some((Self::Fen(fen, halfmove), input))
        }

        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Registration {
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
        && let Some((name, input)) = until_uci("code", input)
        && let Some(input) = literal_uci("code", input) {
            return Some((Self::NameCode(name.join(" "), input.join(" ")), input))
        }

        None
    }
}