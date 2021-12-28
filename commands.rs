
// entry point to cli commands

use super::parse;
use super::gen;
use super::config;

static HELP_MSG: &str = "\
USAGE:
ramen [command]

COMMANDS:
init
add
build
clean
";

pub fn argparse(args: &[String]) {

    let c = args.get(0);
    if c.is_none() {
        help_and_exit()
    }

    match &c.unwrap() as &str {
        "init" => init(),
        "add" => add(),
        "build" => build(&args[1..]),
        "clean" => clean(),
        _ => help_and_exit(),
    };

}

fn help_and_exit() {
    println!("{}", HELP_MSG);
    std::process::exit(1);
}

fn error_and_exit(msg: &str) {
    println!("ERROR: {}", msg);
    std::process::exit(1);
}

fn init() {

}

fn add() {

}

fn build(args: &[String]) {
    if args.len() < 1 {
        help_and_exit()
    }
    let config_dir = &args[0];

    // let user_config = config::load_config(common::DEFAULT_CONFIG_DIR);
    let user_config = config::load_config(config_dir);
    if user_config.is_err() {
        error_and_exit("could not parse config file");
    }
    let user_config = user_config.unwrap();

    let data = parse::parse_all(&user_config.content_path);
    if data.is_err() {
        error_and_exit("error when parsing content dir");
    }
    let data = data.unwrap();

    gen::generate_all(&data, &user_config);

}

fn clean() {
    println!("clean command");
}

