use std::{collections::HashMap, path::PathBuf};

use crate::notation::{
    algebraic::AlgebraicMove,
    pgn::{PGN, PGNHeaders, load_pgn_file},
};
use trie_rs::{self, map};

pub struct Openings {
    file: String,
    trie: trie_rs::map::Trie<AlgebraicMove, PGNAbbrevHeader>,
}

impl Openings {
    pub fn build(file: &str) -> Self {
        let pgns = load_pgn_file(file);

        let mut tb = map::TrieBuilder::new();

        for pgn in pgns {
            tb.insert(
                pgn.move_list(),
                PGNAbbrevHeader::from_pgn_header(&pgn.headers),
            );
        }

        Openings {
            file: file.to_string(),
            trie: tb.build(),
        }
    }
}

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
