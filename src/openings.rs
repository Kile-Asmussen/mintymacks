use std::{
    borrow::Cow,
    collections::HashMap,
    fs::{self, File},
    path::PathBuf,
};

use crate::notation::{
    algebraic::AlgebraicMove,
    pgn::{MovePair, PGN, PGNTags, load_pgn_file},
};
use lazy_static::lazy_static;
use trie_rs::{self, map};

lazy_static! {
    pub static ref OPENINGS_DB: Openings = Openings::build(&*ECO_DB);
}
include_flate::flate!(pub static ECO_DB: str from "eco.pgn");

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

    pub fn from_pgn_header(pgn: &PGNTags) -> Self {
        PGNAbbrevHeader {
            eco: pgn.0.get(&Cow::Borrowed("ECO")).map(Clone::clone),
            opening: pgn.0.get(&Cow::Borrowed("Opening")).map(Clone::clone),
            variation: pgn.0.get(&Cow::Borrowed("Variation")).map(Clone::clone),
        }
    }

    pub fn into_header(self) -> PGNTags {
        let mut res = PGNTags::default();

        if let Some(eco) = self.eco {
            res.0.insert(Cow::Borrowed("ECO"), eco);
        }
        if let Some(opening) = self.opening {
            res.0.insert(Cow::Borrowed("Opening"), opening);
        }
        if let Some(variation) = self.variation {
            res.0.insert(Cow::Borrowed("Variation"), variation);
        }

        res
    }
}
