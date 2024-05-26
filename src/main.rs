use basic_compiler::Config;
use std::env;

use basic_compiler::run;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problems parsing out the arguments: {err}!");
        std::process::exit(1)
    });

    if let Err(e) = run(config) {
        println!("Compilation error: {e}");
        std::process::exit(1)
    }
}
