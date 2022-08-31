extern crate serde;

mod api;
mod commands;
mod common;
mod config;
mod error;
mod gen;
mod parse;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    commands::argparse(&args);
}
