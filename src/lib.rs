#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate crc;

mod crc32;
mod record;

pub mod creation;
pub mod validation;
