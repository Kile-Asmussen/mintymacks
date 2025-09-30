use core::time;
use std::fmt::Write;

use crate::{notation::uci::{Line, Uci}, print_uci};

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
    fn to_string(&self) -> String {
        let mut res = vec![];
        self.print(&mut res);
        res.join(" ")
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
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum PositionString {
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
}