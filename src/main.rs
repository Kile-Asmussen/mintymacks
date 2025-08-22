#![allow(unused)]
#![feature(iter_collect_into)]
#![feature(const_index)]
#![feature(const_trait_impl)]
#![feature(slice_partition_dedup)]

use std::{io::PipeReader, process::Stdio, thread, time::Duration};

use crate::{
    fuzzing::{Stockfish, run_stockfish},
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
    Ok(())
}
