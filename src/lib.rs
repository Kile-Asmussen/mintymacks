#![allow(unused)]
#![feature(iter_collect_into)]
#![feature(const_index)]
#![feature(const_trait_impl)]
#![feature(slice_partition_dedup)]
#![feature(default_field_values)]
#![feature(hash_map_macro)]
#![feature(try_blocks)]
#![feature(adt_const_params)]
#![feature(structural_match)]
#![feature(format_args_nl)]
#![feature(string_from_utf8_lossy_owned)]
#![feature(iter_array_chunks)]
#![feature(portable_simd)]
#![feature(duration_millis_float)]
#![feature(const_option_ops)]
#![feature(hasher_prefixfree_extras)]

use std::{
    alloc::System,
    io::PipeReader,
    process::{Command, Stdio},
    thread,
    time::{Duration, Instant},
};

use crate::{
    bits::{
        board::BitBoard,
        rays::{anti_ray, diag_ray},
    },
    fuzzing::stockfish_perft,
    model::{Square, castling::CastlingRights, moves::ChessMove},
    notation::fen::parse_fen,
};

pub mod arrays;
pub mod bits;
pub mod engine;
pub mod fuzzing;
pub mod model;
#[macro_use]
pub mod notation;
pub mod game;
pub mod openings;
pub mod profile;
pub mod utils;
pub mod zobrist;

#[test]
fn main_perft() {
    BitBoard::startpos().enumerate(6).print();
}
