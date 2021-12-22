
fn main() {

    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() != 2 {
        println!("USAGE: ramen [input dir] [output dir]");
        std::process::exit(1);
    }

    let _input = &args[0];
    let _output = &args[1];

}

