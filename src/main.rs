use clap::Parser;
use std::ffi::OsString;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, stdin};
use std::num::NonZeroUsize;
use std::path::PathBuf;
use std::process::Command;

mod chunks;
use chunks::ChunkIterator;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short = '0', long)]
    null: bool,
    #[arg(short, long)]
    arg_file: Option<PathBuf>,
    #[arg(short, long)]
    delimiter: Option<char>,
    #[arg(short = 'n', long, default_value_t = NonZeroUsize::MAX)]
    max_args: NonZeroUsize,
    program: OsString,
    #[arg(allow_hyphen_values = true)]
    arguments: Vec<OsString>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    match &args.arg_file {
        Some(path) => process(File::open(path)?, &args),
        None => process(stdin(), &args),
    }
}

fn process(source: impl Read, args: &Args) -> anyhow::Result<()> {
    let delim = if let Some(c) = args.delimiter {
        c.try_into()?
    } else if args.null {
        b'\0'
    } else {
        b'\n'
    };
    let input = BufReader::new(source).split(delim);

    for data in input
        .map(|x| x.map(String::from_utf8))
        .map_while(|x| x.ok().and_then(|y| y.ok()))
        .filter(|line| !line.is_empty())
        .chunks(args.max_args.get())
    {
        Command::new(&args.program)
            .args(&args.arguments)
            .args(data)
            .spawn()?
            .wait()?;
    }

    Ok(())
}
