use std::{ error::Error, fs::File, io::{ Read, stdin }, path::{ Path, PathBuf } };

use clap::Parser;
use ortalib::{ Chips, Mult, Round };
use roundscorer::RoundScorer;

mod roundscorer;
mod cardscorer;
mod utils;
mod pokerservice;
mod jokers;
mod modifiers;

#[derive(Parser)]
struct Opts {
    file: PathBuf,

    #[arg(long)]
    explain: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts = Opts::parse();
    let (round, explain) = parse_round(&opts)?;

    let (chips, mult) = score(round, explain);

    println!("{}", (chips * mult).floor());
    Ok(())
}

fn parse_round(opts: &Opts) -> Result<(Round, bool), Box<dyn Error>> {
    let mut input = String::new();
    if opts.file == Path::new("-") {
        stdin().read_to_string(&mut input)?;
    } else {
        File::open(&opts.file)?.read_to_string(&mut input)?;
    }

    let round = serde_yaml::from_str(&input)?;

    // Return both the round and the explain flag
    Ok((round, opts.explain))
}

fn score(round: Round, explain: bool) -> (Chips, Mult) {
    let mut scorer = RoundScorer::new(round);
    scorer.get_score(explain)
}
