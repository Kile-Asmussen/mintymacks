use std::{collections::HashMap, io::Write, iter::Inspect};

pub mod engine;
pub mod gui;
pub mod test;

use crate::model::{moves::PseudoMove, ChessPiece};

pub trait Uci : Sized {
    fn print(&self, output: &mut Vec<String>);

    fn parse_direct<'a>(input: &'a [&'a str]) -> Option<(Self, &'a [&'a str])> {
        None
    }
}

fn parse_uci<'a, U : Uci>(mut input: &'a [&'a str]) -> Option<(U, &'a [&'a str])> {
    loop {
        if let Some(res) = U::parse_direct(input) {
            return Some(res);
        }

        if !input.is_empty() {
            input = &input[1..];
        } else {
            return None;
        }
    }
}

fn parse_until_uci<'a, U : Uci, T : Uci>(mut input: &'a [&'a str]) -> Option<(Vec<U>, &'a [&'a str])> {
    let mut res = vec![];
    loop {
        if input.is_empty() || T::parse_direct(input).is_some() {
            break;
        } else if let Some((val, rest)) = parse_uci(input) {
            input = rest;
            res.push(val);
        } else {
            break;
        }
    }
    Some((res, input))
}

fn parse_many_uci<'a, U : Uci>(mut input: &'a [&'a str]) -> Option<(Vec<U>, &'a [&'a str])> {
        let mut res = vec![];
    loop {
        if input.is_empty() {
            break;
        } else if let Some((val, rest)) = parse_uci(input) {
            input = rest;
            res.push(val);
        } else {
            break;
        }
    }
    Some((res, input))
}


fn literal_uci<'a>(lit: &'a str, mut input: &'a [&'a str]) -> Option<&'a [&'a str]> {
    if input.is_empty() {
            return None;
    } else if input[0] == lit {
        return Some((&input[1..]));
    } else {
        return None;
    }
}

fn find_literal_uci<'a>(lit: &'a str, mut input: &'a [&'a str]) -> Option<&'a [&'a str]> {
    loop {
        if input.is_empty() {
            return None;
        } else if input[0] == lit {
            return Some((&input[1..]));
        }

        input = &input[1..];
    }
}

fn next_uci_token<'a>(input: &'a [&'a str]) -> Option<(String, &'a [&'a str])> {
    if input.is_empty() {
        return None;
    } else {
        return Some((input[0].to_string(), &input[1..]));
    }
}

fn split_at_uci<'a>(stop: &'a str, input: &'a [&'a str]) -> Option<(&'a [&'a str], &'a [&'a str])> {
    if let Some(pos) = input.iter().position(|s| *s == stop) {
        Some((&input[..pos], &input[pos..]))
    } else {
        Some((input, &[]))
    }
}

#[macro_export]
macro_rules! print_uci {
    ($output:ident, $($item:expr),*) => {
        { $( $item.print($output); )* }
    };
}

pub use print_uci;

impl Uci for &str {
    fn print(&self, output: &mut Vec<String>) {
        output.push(self.to_string());
    }
}

impl Uci for String {
    fn print(&self, output: &mut Vec<String>) {
        output.push(self.clone());
    }
}

impl Uci for bool {
    fn print(&self, output: &mut Vec<String>) {
        output.push(self.to_string());
    }

    fn parse_direct<'a>(input: &'a [&'a str]) -> Option<(Self, &'a [&'a str])> {
        if input.is_empty() {
            None
        } else if input[0] == "true" {
            Some((true, &input[1..]))
        } else if input[0] == "false" {
            Some((false, &input[1..]))
        } else {
            None
        }
    }
}

impl Uci for i64 {
    fn print(&self, output: &mut Vec<String>) {
        output.push(self.to_string());
    }

    fn parse_direct<'a>(input: &'a [&'a str]) -> Option<(Self, &'a [&'a str])> {
        if input.is_empty() {
            None
        } else {
            Some((Self::from_str_radix(input[0], 10).ok()?, &input[1..]))
        }
    }
}

impl Uci for u64 {
    fn print(&self, output: &mut Vec<String>) {
        output.push(self.to_string());
    }

    fn parse_direct<'a>(input: &'a [&'a str]) -> Option<(Self, &'a [&'a str])> {
        if input.is_empty() {
            None
        } else {
            Some((Self::from_str_radix(input[0], 10).ok()?, &input[1..]))
        }
    }
}

impl Uci for LongAlg {
    fn print(&self, output: &mut Vec<String>) {
        output.push(self.0.longalg(self.1))
    }

    fn parse_direct<'a>(input: &'a [&'a str]) -> Option<(Self, &'a [&'a str])> {
        if input.is_empty() {
            None
        } else {
            Some((PseudoMove::parse(input[0])?, &input[1..]))
        }
    }
}


impl<U : Uci> Uci for &[U] {
    fn print(&self, output: &mut Vec<String>) {
        for x in *self {
            x.print(output);
        }
    }
}

impl<U : Uci> Uci for Option<U> {
    fn print(&self, output: &mut Vec<String>) {
        match self {
            Self::Some(u) => print_uci!(output, u),
            Self::None => {}
        }
    }
}

pub type LongAlg = (PseudoMove, Option<ChessPiece>);
pub type Line = Vec<LongAlg>;


