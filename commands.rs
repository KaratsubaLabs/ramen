
// entry point to cli commands

use super::parse;
use super::gen;

static HELP_MSG: &str = "\
USAGE:
ramen [command]

COMMANDS:
build
clean
";

pub fn argparse(args: &[String]) {

    let c = args.get(0);
    if c.is_none() {
        help_and_exit()
    }

    match &c.unwrap() as &str {
        "build" => build(&args[1..]),
        "clean" => clean(),
        _ => help_and_exit(),
    };

}

fn help_and_exit() {
    println!("{}", HELP_MSG);
    std::process::exit(1);
}

fn build(args: &[String]) {
    if args.len() < 2 {
        help_and_exit()
    }
    let input = &args[0];
    let output = &args[1];

    println!("build command with {} {}", input, output);
    parse::parse_all(input);

}

fn clean() {
    println!("clean command");
}

