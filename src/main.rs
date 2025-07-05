use clap::Parser;
use std::io::stdin;
use std::num::NonZeroUsize;
use std::process::Command;

mod chunks;
use chunks::ChunkIterator;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short = 'n', long, default_value_t = NonZeroUsize::MAX)]
    max_args: NonZeroUsize,
    program: String,
    #[arg(allow_hyphen_values = true)]
    arguments: Vec<String>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    for lines in stdin()
        .lines()
        .map_while(|x| x.ok())
        .filter(|line| !line.is_empty())
        .chunks(args.max_args.get())
    {
        Command::new(&args.program)
            .args(&args.arguments)
            .args(lines)
            .spawn()?
            .wait()?;
    }

    Ok(())
}
