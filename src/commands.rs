// entry point to cli commands

use std::{
    env, fs,
    fs::File,
    io::prelude::*,
    path::{Path, PathBuf},
};

use crate::common::BoxResult;

#[cfg(feature = "api")]
use super::api;

use super::{config, config::UserConfig, error::CommandError, gen, parse};

use pino_argparse::{Cli, Command, Flag, FlagParse};

static HELP_MSG: &str = "\
USAGE:
ramen [-v] [-c <config>] <command>

COMMANDS:
init
add <anime-name>
meta <anime-name>
build
clean
help
";

type CommandResult<T> = Result<T, CommandError>;

struct Flags {
    pub config_dir: PathBuf,
    pub verbose: bool,
}

impl Flags {
    pub fn new() -> Option<Flags> {
        // by default look for config at ~/.config/ramen/ramenrc
        let mut config_dir = env::home_dir()?;
        config_dir.push(".config");
        config_dir.push("ramen");

        Some(Flags {
            config_dir,
            verbose: false,
        })
    }
}

pub fn run_cli() {
    
    let cli = Cli {
	program_name: "ramen",
	synopsis: "static anime hosting site generator",
	subcommands: vec![
	    Command {
		command_name: "init",
		handler: handle_init,
		..Default::default()
	    },
	    Command {
		command_name: "add",
		handler: handle_add,
		..Default::default()
	    },
	    Command {
		command_name: "meta",
		handler: handle_meta,
		..Default::default()
	    },
	    Command {
		command_name: "build",
		handler: handle_build,
		..Default::default()
	    },
	    Command {
		command_name: "clean",
		handler: handle_clean,
		..Default::default()
	    },
	],
	global_flags: vec![
	    Flag::new("help")
		.short('h'),
	    Flag::new("config")
		.short('c')
	],
	..Default::default()
    };

}

fn handle_init(flagparse: FlagParse) -> BoxResult<()> {
    Ok(())
}

fn handle_add(flagparse: FlagParse) -> BoxResult<()> {
    Ok(())
}

fn handle_meta(flagparse: FlagParse) -> BoxResult<()> {
    Ok(())
}

fn handle_build(flagparse: FlagParse) -> BoxResult<()> {
    Ok(())
}

fn handle_clean(flagparse: FlagParse) -> BoxResult<()> {
    Ok(())
}

pub fn argparse(args: &[String]) {
    let mut mut_args = args.to_vec();
    match dispatch_command(&mut mut_args) {
        Ok(_) => (),
        Err(e) => {
            if e.display_help {
                let _ = help();
            }
            if e.msg.is_some() {
                println!("{}", e.msg.unwrap());
            }
            std::process::exit(1);
        },
    }
}

fn dispatch_command(args: &mut Vec<String>) -> CommandResult<()> {
    let flags = parse_flags(args)?;
    let c: &str = &args.get(0).ok_or(CommandError::with_help())?.clone();
    args.remove(0);

    let user_config = config::load_config(&flags.config_dir)
        .or(Err(CommandError::with_error("could not parse config file")))?;

    match c {
        "init" => init(&flags, &user_config),
        "add" => add(args, &flags, &user_config),
        "meta" => meta(args, &flags),
        "build" => build(&flags, &user_config),
        "clean" => clean(&flags),
        "help" => help(),
        _ => Err(CommandError::with_help()),
    }?;

    Ok(())
}

fn parse_flags(args: &mut Vec<String>) -> CommandResult<Flags> {
    let mut flags = Flags::new().ok_or(CommandError::with_error("could not init flags"))?;

    while args.len() > 0 {
        match &args[0] as &str {
            "-c" => {
                args.remove(0);
                let config_dir = args.get(0).ok_or(CommandError::with_help())?;
                flags.config_dir = PathBuf::from(config_dir);
            },
            "-v" => flags.verbose = true,
            _ => break,
        }
        args.remove(0);
    }

    Ok(flags)
}

fn init(flags: &Flags, user_config: &UserConfig) -> CommandResult<()> {
    Ok(())
}

fn add(args: &mut Vec<String>, flags: &Flags, user_config: &UserConfig) -> CommandResult<()> {
    let anime_name: &str = &args.get(0).ok_or(CommandError::with_help())?.clone();
    args.remove(0);

    let anime_path = &user_config.content_path.join(anime_name);
    if anime_path.is_dir() {
        return Err(CommandError::with_error(&format!(
            "anime with name {} already exists",
            &anime_name
        )));
    }

    fs::create_dir(&anime_path).or(Err(CommandError::with_error(&format!(
        "could not create dir for anime {}",
        &anime_name
    ))))?;
    fs::create_dir(anime_path.join("data"))
        .or(Err(CommandError::with_error("could not create data dir")))?;
    fs::create_dir(anime_path.join("files"))
        .or(Err(CommandError::with_error("could not create files dir")))?;
    fs::create_dir(anime_path.join("subtitles")).or(Err(CommandError::with_error(
        "could not create subtitles dir",
    )))?;

    let mut f = File::create(&anime_path.join("data").join("metadata")).or(Err(
        CommandError::with_error("could not create metadata file"),
    ))?;

    let default_meta = format!(
        r"
title      = {anime_name}
synopsis   = synopsis
anime_type = tv
",
        anime_name = anime_name
    );

    f.write_all(default_meta.as_bytes())
        .or(Err(CommandError::with_error(
            "could not write default metadata file",
        )))?;

    Ok(())
}

fn meta(args: &mut Vec<String>, flags: &Flags) -> CommandResult<()> {
    #[cfg(feature = "api")]
    {
    let anime_name: &str = &args.get(0).ok_or(CommandError::with_help())?.clone();
    args.remove(0);

    let anime_meta = api::get_anime_by_id(anime_name)
        .or(Err(CommandError::with_error("error making api request")))?;

    // println!("{:?}", anime_meta);

    gen::generate_meta_file(anime_meta);
    }
    #[cfg(not(feature = "api"))]
    {
        return Err(CommandError::with_error("api feature not enabled"))
    }

    Ok(())
}

fn build(flags: &Flags, user_config: &UserConfig) -> CommandResult<()> {
    let data = parse::parse_all(&user_config.content_path).or(Err(CommandError::with_error(
        "error when parsing content dir",
    )))?;

    gen::generate_all(&data, &user_config).or(Err(CommandError::with_error(
        "error when generating static files",
    )))?;

    Ok(())
}

fn clean(flags: &Flags) -> CommandResult<()> {
    Ok(())
}

fn help() -> CommandResult<()> {
    println!("{}", HELP_MSG);
    Ok(())
}
