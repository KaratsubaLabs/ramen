
mod commands;
mod common;
mod config;
mod gen;
mod parse;
mod error;

fn main() {

    let args: Vec<String> = std::env::args().skip(1).collect();
    match commands::argparse(&args) {
        Ok(_)  => (), 
        Err(e) => {
            println!("{}", e);
            std::process::exit(1); 
        }
    }

}

