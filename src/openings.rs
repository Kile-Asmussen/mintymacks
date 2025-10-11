use std::{
    collections::HashMap,
    fs::{self, File},
    path::PathBuf,
};

use crate::notation::{
    algebraic::AlgebraicMove,
    pgn::{MovePair, PGN, PGNHeaders, load_pgn_file},
};
use trie_rs::{self, map};

pub struct Openings {
    pgns: Vec<PGN>,
    trie: trie_rs::map::Trie<AlgebraicMove, PGNAbbrevHeader>,
}

impl Openings {
    pub fn build(file: &str) -> Self {
        let pgns = load_pgn_file(file);

        let mut tb = map::TrieBuilder::new();

        for pgn in &pgns {
            tb.insert(
                pgn.move_list(),
                PGNAbbrevHeader::from_pgn_header(&pgn.headers),
            );
        }

        Openings {
            pgns,
            trie: tb.build(),
        }
    }
}

#[derive(Debug)]
pub struct PGNAbbrevHeader {
    eco: Option<String>,
    opening: Option<String>,
    variation: Option<String>,
}

impl PGNAbbrevHeader {
    pub fn new(eco: Option<String>, opening: Option<String>, variation: Option<String>) -> Self {
        PGNAbbrevHeader {
            eco,
            opening,
            variation,
        }
    }

    pub fn from_pgn_header(pgn: &PGNHeaders) -> Self {
        PGNAbbrevHeader {
            eco: pgn.eco.clone(),
            opening: pgn.opening.clone(),
            variation: pgn.variation.clone(),
        }
    }

    pub fn into_header(self) -> PGNHeaders {
        let mut res = PGNHeaders::default();

        res.eco = self.eco;
        res.opening = self.opening;
        res.variation = self.variation;

        res
    }
}

#[test]
fn build_openings() {
    let file = String::from_utf8_lossy_owned(fs::read("eco.pgn").unwrap());

    let op = Openings::build(&file);

    let query = &[];
    for (pf, pgn) in op.trie.postfix_search::<Vec<AlgebraicMove>, _>(query) {
        let pf = MovePair::pair_moves(query.iter().chain(pf.iter()).map(|a| *a));

        println!(
            "{} {{{}: {} {}}}",
            pf.iter()
                .map(|mp| mp.to_string())
                .collect::<Vec<_>>()
                .join(" "),
            pgn.eco.as_deref().unwrap_or(""),
            pgn.opening.as_deref().unwrap_or(""),
            pgn.variation.as_deref().unwrap_or(""),
        )
    }
}
