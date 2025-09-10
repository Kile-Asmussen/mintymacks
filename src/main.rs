#![allow(unused)]
#![feature(iter_collect_into)]
#![feature(const_index)]
#![feature(const_trait_impl)]
#![feature(slice_partition_dedup)]
#![feature(default_field_values)]

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
    model::{Square, castling::CastlingRights, moves::Move},
};

mod arrays;
mod bits;
mod eval;
mod fuzzing;
mod minmax;
mod model;
mod uci;
mod zobrist;

#[global_allocator]
static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

fn main() {
    BitBoard::startpos().perft(8).print();
}
