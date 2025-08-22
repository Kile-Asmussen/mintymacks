#![allow(unused)]
#![feature(iter_collect_into)]
#![feature(const_index)]
#![feature(const_trait_impl)]
#![feature(slice_partition_dedup)]

use std::{
    io::PipeReader,
    process::{Command, Stdio},
    thread,
    time::{Duration, Instant},
};

use crate::model::{Square, castling::CastlingRights, moves::Move};

mod arrays;
mod bits;
mod minmax;
mod model;
mod uci;
mod zobrist;

fn main() -> anyhow::Result<()> {
    let t0 = Instant::now();
    let res = String::from_utf8(
        Command::new("./stockfish-perft.sh")
            .arg("")
            .arg("1")
            .output()?
            .stdout,
    )?
    .trim_end()
    .to_string();

    println!("{}", t0.elapsed().as_millis());
    println!("{}", res);

    Ok(())
}
