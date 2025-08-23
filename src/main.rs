#![allow(unused)]
#![feature(iter_collect_into)]
#![feature(const_index)]
#![feature(const_trait_impl)]
#![feature(slice_partition_dedup)]
#![feature(default_field_values)]

use std::{
    io::PipeReader,
    process::{Command, Stdio},
    thread,
    time::{Duration, Instant},
};

use crate::{
    fuzzing::stockfish_perft,
    model::{Square, castling::CastlingRights, moves::Move},
};

mod arrays;
mod bits;
mod fuzzing;
mod minmax;
mod model;
mod uci;
mod zobrist;

fn main() -> anyhow::Result<()> {
    println!("{:?}", stockfish_perft(&[], 5)?);

    Ok(())
}
