use std::env;
use std::fs;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("No more than 1 argument is accepted!");
        exit(1)
    }

    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Failed!");

    println!("{:?}", contents);
}
