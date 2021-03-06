
extern crate serde;

mod commands;
mod common;
mod config;
mod gen;
mod parse;
mod error;
mod api;

fn main() {

    let args: Vec<String> = std::env::args().skip(1).collect();
    commands::argparse(&args);

}

