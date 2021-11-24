use mychip8::cli::Cli;

use structopt::StructOpt;

fn main() {
    let args = Cli::from_args();
    if !args.is_game_file_existed() {
        eprintln!("ERROR: Game file not found.");
        std::process::exit(1);
    }

    if let Err(err) = mychip8::run(args) {
        eprintln!("ERROR: {}.", err);
        if let Some(source) = err.source() {
            eprintln!("SOURCE: {}.", source);
        }
        std::process::exit(1);
    }
}
