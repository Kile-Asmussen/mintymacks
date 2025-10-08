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

use crate::{
    bits::board::BitBoard,
    fuzzing::stockfish_perft,
    model::{Square, castling::CastlingRights, moves::ChessMove},
};

pub mod arrays;
pub mod bits;
pub mod fuzzing;
pub mod model;
#[macro_use]
pub mod notation;
pub mod zobrist;
