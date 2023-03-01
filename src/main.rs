use clap::{value_parser, Arg, Command};
use prio::field::Field64;

use vdaf_rejection_sampling_search::{search, Config};

fn main() {
    let matches = app().get_matches();
    let jobs = *matches.get_one("jobs").unwrap();
    let prg_iterations = *matches.get_one("prg-iterations").unwrap();
    let config = Config::new(jobs, prg_iterations);
    search::<Field64>(config);
}

fn app() -> Command {
    Command::new("vdaf-rejection-sampling-search")
        .arg(
            Arg::new("jobs")
                .short('j')
                .long("jobs")
                .value_parser(value_parser!(usize))
                .default_value("1"),
        )
        .arg(
            Arg::new("prg-iterations")
                .short('n')
                .long("prg-iterations")
                .value_parser(value_parser!(usize))
                .default_value("100000"),
        )
}
