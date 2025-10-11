use std::{
    collections::HashMap,
    fs::{self, File},
    path::PathBuf,
};

use crate::notation::{
    algebraic::AlgebraicMove,
    pgn::{MovePair, PGN, PGNTags, Tag, load_pgn_file},
};
use lazy_static::lazy_static;
use trie_rs::{self, map};

lazy_static! {
    pub static ref OPENINGS_DB: Openings = Openings::build(&*ECO_DB);
};
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
            eco: pgn.canon.get(&Tag::ECO).map(Clone::clone),
            opening: pgn.canon.get(&Tag::Opening).map(Clone::clone),
            variation: pgn.canon.get(&Tag::Variation).map(Clone::clone),
        }
    }

    pub fn into_header(self) -> PGNTags {
        let mut res = PGNTags::default();

        if let Some(eco) = self.eco {
            res.canon.insert(Tag::ECO, eco);
        }
        if let Some(opening) = self.opening {
            res.canon.insert(Tag::Opening, opening);
        }
        if let Some(variation) = self.variation {
            res.canon.insert(Tag::Variation, variation);
        }

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
