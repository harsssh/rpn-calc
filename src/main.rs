mod calculator;

use calculator::RpnCalculator;
use clap::Parser;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use anyhow::Result;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(value_name = "FILE")]
    formula_file: Option<String>,

    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if let Some(path) = args.formula_file {
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);
        run(reader, args.verbose)
    } else {
        let stdin = stdin();
        let reader = stdin.lock();
        run(reader, args.verbose)
    }
}

fn run<R: BufRead>(reader: R, verbose: bool) -> Result<()> {
    let calc = RpnCalculator::new(verbose);

    for line in reader.lines() {
        let line = line?;
        match calc.eval(&line) {
            Ok(answer) => println!("{}", answer),
            Err(e) => eprintln!("{:#?}", e)
        }
    }

    Ok(())
}

