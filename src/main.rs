#![allow(unused)]
#![feature(iter_collect_into)]
#![feature(const_index)]
#![feature(const_trait_impl)]
#![feature(slice_partition_dedup)]
#![feature(default_field_values)]
#![feature(hash_map_macro)]
#![feature(try_blocks)]
#![feature(adt_const_params)]

use std::{
    alloc::System,
    io::PipeReader,
    process::{Command, Stdio},
    thread,
    time::{Duration, Instant},
};

use stats_alloc::{INSTRUMENTED_SYSTEM, Region, StatsAlloc};

use crate::{
    bits::board::BitBoard,
    fuzzing::stockfish_perft,
    model::{Square, castling::CastlingRights, moves::ChessMove},
};

mod arrays;
mod bits;
mod fuzzing;
mod model;
#[macro_use]
mod notation;
mod game;
mod zobrist;

#[global_allocator]
static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

fn main() {
    BitBoard::startpos().enumerate(7).print();
}
