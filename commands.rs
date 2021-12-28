
// entry point to cli commands

use super::parse;
use super::gen;
use super::config;
use super::error::{CommandError};

static HELP_MSG: &str = "\
USAGE:
ramen [command]

COMMANDS:
init
add
build
clean
help
";

pub type CommandResult = Result<(), CommandError>;

pub fn argparse(args: &[String]) -> CommandResult {

    let c = args.get(0).ok_or(CommandError::with_help())?;

    match c as &str {
        "init"  => init(),
        "add"   => add(),
        "build" => build(&args[1..]),
        "clean" => clean(),
        "help"  => help(),
        _       => Err(CommandError::with_help())
    }?;

    Ok(())
}

fn init() -> CommandResult {

    Ok(())
}

fn add() -> CommandResult {

    Ok(())
}

fn build(args: &[String]) -> CommandResult {
    if args.len() < 1 {
        return Err(CommandError::with_help());
    }
    let config_dir = &args[0];

    // let user_config = config::load_config(common::DEFAULT_CONFIG_DIR);
    let user_config = config::load_config(config_dir)
        .or(Err(CommandError::with_error("could not parse config file")))?;

    let data = parse::parse_all(&user_config.content_path)
        .or(Err(CommandError::with_error("error when parsing content dir")))?;

    gen::generate_all(&data, &user_config)
        .or(Err(CommandError::with_error("error when generating static files")))?;

    Ok(())
}

fn clean() -> CommandResult {
    println!("clean command");
    Ok(())
}

fn help() -> CommandResult {
    println!("{}", HELP_MSG);
    Ok(())
}

