
// entry point to cli commands

use std::env;
use std::path::{Path, PathBuf};

use super::parse;
use super::gen;
use super::config;
use super::error::{CommandError};

static HELP_MSG: &str = "\
USAGE:
ramen [-v] [-c <config>] <command>

COMMANDS:
init
add
build
clean
help
";

type CommandResult<T> = Result<T, CommandError>;

struct Flags {
    pub config_dir: PathBuf,
    pub verbose: bool
}

impl Flags {
    pub fn new() -> Option<Flags> {
        // by default look for config at ~/.config/ramen/ramenrc
        let mut config_dir = env::home_dir()?;
        config_dir.push(".config");
        config_dir.push("ramen");

        Some(Flags{
            config_dir: config_dir,
            verbose: false
        })
    }
}

pub fn argparse(args: &[String]) {

    let mut mut_args = args.to_vec();
    match dispatch_command(&mut mut_args) {
        Ok(_)  => (), 
        Err(e) => {
            if e.display_help {
                let _ = help();
            }
            if e.msg.is_some() {
                println!("{}", e.msg.unwrap());
            }
            std::process::exit(1); 
        }
    }
}

fn dispatch_command(args: &mut [String]) -> CommandResult<()> {

    let flags = parse_flags(args)?;
    let c = args.get(0).ok_or(CommandError::with_help())?;

    match c as &str {
        "init"  => init(&flags),
        "add"   => add(&flags),
        "build" => build(&flags),
        "clean" => clean(&flags),
        "help"  => help(),
        _       => Err(CommandError::with_help())
    }?;

    Ok(())
}

fn parse_flags(mut args: &mut [String]) -> CommandResult<Flags> {
    
    let mut flags = Flags::new().ok_or(CommandError::with_error("could not init flags"))?;

    while args.len() > 0 {
        match &args[0] as &str {
            "-c" => {
                args = &mut args[1..];
                let config_dir = args.get(0).ok_or(CommandError::with_help())?;
                flags.config_dir = PathBuf::from(config_dir);
            },
            "-v" => flags.verbose = true,
            _    => () // should send help message
        }
        args = &mut args[1..];
    }

    Ok(flags)
}

fn init(flags: &Flags) -> CommandResult<()> {

    Ok(())
}

fn add(flags: &Flags) -> CommandResult<()> {

    Ok(())
}

fn build(flags: &Flags) -> CommandResult<()> {

    // let user_config = config::load_config(common::DEFAULT_CONFIG_DIR);
    let user_config = config::load_config(&flags.config_dir)
        .or(Err(CommandError::with_error("could not parse config file")))?;

    let data = parse::parse_all(&user_config.content_path)
        .or(Err(CommandError::with_error("error when parsing content dir")))?;

    gen::generate_all(&data, &user_config)
        .or(Err(CommandError::with_error("error when generating static files")))?;

    Ok(())
}

fn clean(flags: &Flags) -> CommandResult<()> {

    Ok(())
}

fn help() -> CommandResult<()> {
    println!("{}", HELP_MSG);
    Ok(())
}

