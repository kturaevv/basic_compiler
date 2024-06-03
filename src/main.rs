use basic_compiler::Config;
use std::env;

use basic_compiler::run;

fn setup_tracing(enable: bool) {
    if enable {
        tracing_subscriber::fmt()
            // enable everything
            .with_max_level(tracing::Level::TRACE)
            .compact()
            .pretty()
            // Display source code file paths
            .with_file(true)
            // Display source code line numbers
            .with_line_number(true)
            // Display the thread ID an event was recorded on
            // .with_thread_ids(true)
            // Don't display the event's target (module path)
            .with_target(false)
            // sets this to be the default, global collector for this application.
            .init();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let enable_tracing = args.contains(&String::from("--debug"));

    setup_tracing(enable_tracing);

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problems parsing out the arguments: {err}!");
        std::process::exit(1)
    });

    if let Err(e) = run(config) {
        println!("Compilation error: {e}");
        std::process::exit(1)
    }
}
