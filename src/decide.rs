use std::{error::Error, io::{BufRead, BufReader, Write}};
use std::fs::OpenOptions;
use clap::Parser;
use fractran::program::program::FractranProgram;
use std::fs::File;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// List of fractran programs
    #[arg(short, long)]
    infile: String,

    /// Output with decisions
    #[arg(short, long)]
    outfile: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let file = File::open(args.infile)?;
    let mut outfile = OpenOptions::new().append(true).create(true).open(args.outfile)?;
    let mut reader = BufReader::new(file).lines();

    for line in reader {
        match line {
            Ok(prog) => {
                outfile.write(FractranProgram::from(prog).get_decision_text().as_bytes());
                outfile.write(b"\n");
            }
            _ => { outfile.write(b"-----------------------------------------\n"); }
        }
    }

    Ok(())
}