use std::ops::Deref;

use crate::{notation::uci::{literal_uci, parse_uci, token_uci, Line, LongAlg, Uci}, print_uci, regexp};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UciEngine {
    CopyProtection(AuthResult),
    Registration(AuthResult),
    BestMove(BestMove),
    Id(IdString),
    Info(Vec<InfoString>),
    Option(EngineOption),
    ReadyOk(),
    UciOk(),
}

impl UciEngine {
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

impl Uci for UciEngine {
    fn print(&self, output: &mut Vec<String>) {
        match self {
            Self::CopyProtection(auth) => print_uci!(output, "copyprotection", auth),
            Self::Registration(auth) => print_uci!(output, "registration", auth),
            Self::BestMove(best_move) => print_uci!(output, "bestmove", best_move),
            Self::Id(id_string) => print_uci!(output, "id", id_string),
            Self::Info(info_strings) => print_uci!(output, "info", info_strings),
            Self::Option(uci_option) => print_uci!(output, "option", uci_option),
            Self::ReadyOk() => print_uci!(output, "readyok"),
            Self::UciOk() => print_uci!(output, "uciok"),
        }
    }

    fn parse_direct<'a>(input: &'a [&'a str]) -> Option<(Self, &'a [&'a str])> {
        if let Some(input) = literal_uci("readyok", input) {
            return Some((Self::ReadyOk(), input));
        }

        if let Some(input) = literal_uci("uciok", input) {
            return Some((Self::UciOk(), input));
        }

        if let Some(input) = literal_uci("option", input)
        && let Some((option, input)) = parse_uci(input) {
            return Some((Self::Option(option), input));
        }

        if let Some(input) = literal_uci("info", input)
        && let Some((option, input)) = parse_uci(input) {
            return Some((Self::Info(option), input));
        }

        if let Some(input) = literal_uci("id", input)
        && let Some((option, input)) = parse_uci(input) {
            return Some((Self::Id(option), input));
        }

        if let Some(input) = literal_uci("bestmove", input)
        && let Some((option, input)) = parse_uci(input) {
            return Some((Self::BestMove(option), input));
        }

        if let Some(input) = literal_uci("registration", input)
        && let Some((option, input)) = parse_uci(input) {
            return Some((Self::Registration(option), input));
        }

        if let Some(input) = literal_uci("copyprotection", input)
        && let Some((option, input)) = parse_uci(input) {
            return Some((Self::CopyProtection(option), input));
        }

        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthResult {
    Checking,
    Error,
    Ok
}

impl Uci for AuthResult {
    fn print(&self, output: &mut Vec<String>) {
        print_uci!(output, match self {
            Self::Checking => "checking",
            Self::Error => "error",
            Self::Ok => "ok",
        });
    }

    fn parse_direct<'a>(input: &'a [&'a str]) -> Option<(Self, &'a [&'a str])> {
        if let Some(input) = literal_uci("checking", input) {
            return Some((Self::Checking, input));
        }

        if let Some(input) = literal_uci("error", input) {
            return Some((Self::Error, input));
        }

        if let Some(input) = literal_uci("ok", input) {
            return Some((Self::Ok, input));
        }

        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BestMove {
    best: LongAlg,
    ponder: Option<LongAlg>,
}

impl Uci for BestMove {
    fn print(&self, output: &mut Vec<String>) {
        print_uci!(output, self.best);
        if let Some(ponder) = self.ponder {
            print_uci!(output, "ponder", ponder);
        }
    }

    fn parse_direct<'a>(input: &'a [&'a str]) -> Option<(Self, &'a [&'a str])> {
        let (best, input) = parse_uci(input)?;
        
        let (ponder, input) =
        if let Some(input) = literal_uci("ponder", input)
        && let Some((mv, input)) = parse_uci(input) {
            (Some(mv), input)
        } else {
            (None, input)
        };

        Some((Self { best, ponder }, input))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IdString {
    Name(String),
    Author(String),
}

impl Uci for IdString {
    fn print(&self, output: &mut Vec<String>) {
        match self {
            Self::Name(n) => print_uci!(output, "name", n),
            Self::Author(a) => print_uci!(output, "author", a),
        }
    }

    fn parse_direct<'a>(input: &'a [&'a str]) -> Option<(Self, &'a [&'a str])> {
        if let Some(input) = literal_uci("name", input) {
            return Some((Self::Name(input.join(" ")), &[]));
        }

        if let Some(input) = literal_uci("author", input) {
            return Some((Self::Author(input.join(" ")), &[]));
        }

        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InfoString {
    Depth(u64),
    SelDepth(u64),
    Time(u64),
    Nodes(u64),
    MultiVariation(u64),
    PrincipleVariation(Line),
    Score(ScoreBound, ScoreString),
    CurrentMove(LongAlg),
    CurrentMoveNumber(u64),
    HashFullPermill(u64),
    NodesPerSecond(u64),
    TableBaseHits(u64),
    ShredderTableBaseHits(u64),
    CpuLoadPermill(u64),
    Refutation(LongAlg, Line),
    CurrLine(u64, Line),
}

impl Uci for InfoString {
    fn print(&self, output: &mut Vec<String>) {
        match self {
            Self::Depth(n) => print_uci!(output, "depth", n),
            Self::SelDepth(n) => print_uci!(output, "seldepth", n),
            Self::Time(n) => print_uci!(output, "time", n),
            Self::Nodes(n) => print_uci!(output, "nodes", n),
            Self::MultiVariation(n) => print_uci!(output, "multipv", n),
            Self::PrincipleVariation(line) => print_uci!(output, "pv", line),
            Self::Score(score_bound, score_string) => print_uci!(output, "score", score_bound, score_string),
            Self::CurrentMove(mv) => print_uci!(output, "currmove", mv),
            Self::CurrentMoveNumber(n) => print_uci!(output, "currmovenumber", n),
            Self::HashFullPermill(n) => print_uci!(output, "hashfull", n),
            Self::NodesPerSecond(n) => print_uci!(output, "nps", n),
            Self::TableBaseHits(n) => print_uci!(output, "tbhits", n),
            Self::ShredderTableBaseHits(n) => print_uci!(output, "sbhits", n),
            Self::CpuLoadPermill(n) => print_uci!(output, "cpuload", n),
            Self::Refutation(mv, line) => print_uci!(output, "refutation", mv, line),
            Self::CurrLine(cpu, line) => print_uci!(output, "currline", if *cpu != 0 { Some(*cpu) } else { None }, line),
        }
    }

    fn parse_direct<'a>(input: &'a [&'a str]) -> Option<(Self, &'a [&'a str])> {
        if let Some(input) = literal_uci("depth", input)
        && let Some((n, input)) = parse_uci(input) {
            return Some((Self::Depth(n), input))
        }

        if let Some(input) = literal_uci("seldepth", input)
        && let Some((n, input)) = parse_uci(input) {
            return Some((Self::SelDepth(n), input))
        }

        if let Some(input) = literal_uci("time", input)
        && let Some((n, input)) = parse_uci(input) {
            return Some((Self::Time(n), input))
        }

        if let Some(input) = literal_uci("nodes", input)
        && let Some((n, input)) = parse_uci(input) {
            return Some((Self::Nodes(n), input))
        }

        if let Some(input) = literal_uci("multipv", input)
        && let Some((n, input)) = parse_uci(input) {
            return Some((Self::MultiVariation(n), input))
        }

        if let Some(input) = literal_uci("score", input)
        && let Some((sb, input)) = parse_uci(input)
        && let Some((ss, input)) = parse_uci(input) {
            return Some((Self::Score(sb, ss), input))
        }

        if let Some(input) = literal_uci("pv", input)
        && let Some((pv, input)) = parse_uci(input) {
            return Some((Self::PrincipleVariation(pv), input))
        }

        if let Some(input) = literal_uci("currmove", input)
        && let Some((mv, input)) = parse_uci(input) {
            return Some((Self::CurrentMove(mv), input))
        }

        if let Some(input) = literal_uci("hashfull", input)
        && let Some((n, input)) = parse_uci(input) {
            return Some((Self::HashFullPermill(n), input))
        }

        if let Some(input) = literal_uci("nps", input)
        && let Some((n, input)) = parse_uci(input) {
            return Some((Self::NodesPerSecond(n), input))
        }

        if let Some(input) = literal_uci("tbhits", input)
        && let Some((n, input)) = parse_uci(input) {
            return Some((Self::TableBaseHits(n), input))
        }

        if let Some(input) = literal_uci("sbhits", input)
        && let Some((n, input)) = parse_uci(input) {
            return Some((Self::ShredderTableBaseHits(n), input))
        }

        if let Some(input) = literal_uci("cpuload", input)
        && let Some((n, input)) = parse_uci(input) {
            return Some((Self::CpuLoadPermill(n), input))
        }

        if let Some(input) = literal_uci("refutation", input)
        && let Some((mv, input)) = parse_uci(input)
        && let Some((line, input)) = parse_uci(input) {
            return Some((Self::Refutation(mv, line), input))
        }

        if let Some(input) = literal_uci("currline", input)
        && let Some((cpu, input)) = parse_uci::<Option<u64>>(input)
        && let Some((line, input)) = parse_uci(input) {
            return Some((Self::CurrLine(cpu.unwrap_or(0), line), input))
        }

        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScoreBound {
    Upper, Lower, Precise
}

impl Uci for ScoreBound {
    fn print(&self, output: &mut Vec<String>) {
        match self {
            Self::Upper => print_uci!(output, "upperbound"),
            Self::Lower => print_uci!(output, "lowerbound"),
            Self::Precise => {}
        }
    }

    fn parse_direct<'a>(input: &'a [&'a str]) -> Option<(Self, &'a [&'a str])> {
        if let Some(input) = literal_uci("upperbound", input) {
            Some((Self::Upper, input))
        } else if let Some(input) = literal_uci("lowerbound", input) {
            Some((Self::Lower, input))
        } else {
            Some((Self::Precise, input))
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScoreString {
    Centipawns(i64),
    MateIn(u64),
}

impl Uci for ScoreString {
    fn print(&self, output: &mut Vec<String>) {
        match self {
            Self::Centipawns(n) => print_uci!(output, "cp", n),
            Self::MateIn(n) => print_uci!(output, "mate", n),
        }
    }

    fn parse_direct<'a>(input: &'a [&'a str]) -> Option<(Self, &'a [&'a str])> {
        if let Some(input) = literal_uci("cp", input)
        && let Some((n, input)) = parse_uci(input) {
            return Some((Self::Centipawns(n), input))
        }

        if let Some(input) = literal_uci("mate", input)
        && let Some((n, input)) = parse_uci(input) {
            return Some((Self::MateIn(n), input))
        }

        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EngineOption {
    pub name: String,
    pub option_type: OptionType,
}

impl Uci for EngineOption {
    fn print(&self, output: &mut Vec<String>) {
        print_uci!(output, "name", self.name, self.option_type)
    }

    fn parse_direct<'a>(input: &'a [&'a str]) -> Option<(Self, &'a [&'a str])> {
        let input = literal_uci("name", input)?;
        let (name, input) = token_uci(input)?;
        let (option_type, input) = parse_uci(input)?;

        Some((Self { name, option_type }, input))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OptionType {
    Check(CheckType),
    Spin(SpinType),
    Combo(ComboType),
    Button(ButtonType),
    String(StringType)
}

impl Uci for OptionType {
    fn print(&self, output: &mut Vec<String>) {
        match self {
            Self::Check(t) => print_uci!(output, "check", t),
            Self::Spin(t) => print_uci!(output, "spin", t),
            Self::Combo(t) => print_uci!(output, "combo", t),
            Self::Button(t) => print_uci!(output, "button", t),
            Self::String(t) => print_uci!(output, "string", t),
        }
    }

    fn parse_direct<'a>(input: &'a [&'a str]) -> Option<(Self, &'a [&'a str])> {
        
        if let Some(input) = literal_uci("check", input)
        && let Some((t, input)) = parse_uci(input) {
            return Some((Self::Check(t), input))
        }

        if let Some(input) = literal_uci("spin", input)
        && let Some((t, input)) = parse_uci(input) {
            return Some((Self::Spin(t), input))
        }


        if let Some(input) = literal_uci("combo", input)
        && let Some((t, input)) = parse_uci(input) {
            return Some((Self::Combo(t), input))
        }

        if let Some(input) = literal_uci("button", input)
        && let Some((t, input)) = parse_uci(input) {
            return Some((Self::Button(t), input))
        }

        if let Some(input) = literal_uci("string", input)
        && let Some((t, input)) = parse_uci(input) {
            return Some((Self::String(t), input))
        }

        return None;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CheckType {
    default: bool,
    value: Option<bool>
}

impl Uci for CheckType {
    fn print(&self, output: &mut Vec<String>) {
        print_uci!(output, "default", (if self.default { "true" } else { "false" }))
    }

    fn parse_direct<'a>(input: &'a [&'a str]) -> Option<(Self, &'a [&'a str])> {
        let input = literal_uci("default", input)?;
        let (default, input) = parse_uci(input)?;

        Some((Self { default, value: None }, input))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpinType {
    default: i64,
    min: i64,
    max: i64,
    value: Option<i64>
}

impl Uci for SpinType {
    fn print(&self, output: &mut Vec<String>) {
        print_uci!(output, "default", self.default, "min", self.min, "max", self.max);
    }

    fn parse_direct<'a>(input: &'a [&'a str]) -> Option<(Self, &'a [&'a str])> {
        let input1 = literal_uci("default", input)?;
        let (default, input1) = parse_uci(input)?;

        let input2 = literal_uci("min", input)?;
        let (min, input2) = parse_uci(input2)?;

        let input3 = literal_uci("min", input)?;
        let (max, input3) = parse_uci(input3)?;

        let inputs = [input1, input2, input3];
        let input = inputs.iter().min_by_key(|s| s.len())?;

        Some((Self { default, min, max, value: None }, input))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComboType {
    default: String,
    variants: Vec<String>,
}

impl Uci for ComboType {
    fn print(&self, output: &mut Vec<String>) {
        print_uci!(output, "default", self.default);
        for var in &self.variants {
            print_uci!(output, "var", var);
        }
    }

    fn parse_direct<'a>(input: &'a [&'a str]) -> Option<(Self, &'a [&'a str])> {
        
        let mut variants = vec![];
        let mut input1 = input;
        while let Some(rest) = literal_uci("var", input)
        && let Some((var, rest)) = token_uci(input) {
            variants.push(var);
            input1 = rest;
        }

        if variants.is_empty() {
            return None;
        }

        let input2 = literal_uci("default", input)?;
        let (default, input2) = token_uci(input2)?;

        let input = if input1.len() < input2.len() {
            input1
        } else {
            input2
        };

        Some((Self {
            variants, default
        }, input))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringType {
    default: String,
    value: Option<String>,
}

impl Uci for StringType {
    fn print(&self, output: &mut Vec<String>) {
        print_uci!(output, self.default);
    }

    fn parse_direct<'a>(input: &'a [&'a str]) -> Option<(Self, &'a [&'a str])> {
        Some((Self { default: input.join(" "), value: None }, &[]))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ButtonType;

impl Uci for ButtonType {
    fn print(&self, output: &mut Vec<String>) {
        
    }

    fn parse_direct<'a>(input: &'a [&'a str]) -> Option<(Self, &'a [&'a str])> {
        Some((Self, input))
    }
}