
static HELP_MSG: &str = "\
USAGE:
ramen [command]

COMMANDS:
build
clean
";

fn help_and_exit() {
    println!("{}", HELP_MSG);
    std::process::exit(1);
}

fn build_command(args: &[String]) {
    if args.len() < 2 {
        help_and_exit()
    }
    let input = &args[0];
    let output = &args[1];

    println!("build command with {} {}", input, output);
}

fn clean_command() {
    println!("clean command");
}

fn argparse(args: &[String]) {

    let c = args.get(0);
    if c.is_none() {
        help_and_exit()
    }

    match &c.unwrap() as &str {
        "build" => build_command(&args[1..]),
        "clean" => clean_command(),
        _ => help_and_exit(),
    };

}

fn main() {

    let args: Vec<String> = std::env::args().skip(1).collect();
    argparse(&args);
}

