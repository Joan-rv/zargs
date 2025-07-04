use clap::Parser;
use std::io::stdin;
use std::process::Command;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short = 'n', long, default_value_t = usize::MAX)]
    max_args: usize,
    program: String,
    #[arg(allow_hyphen_values = true)]
    arguments: Vec<String>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    for lines in stdin()
        .lines()
        .collect::<Result<Vec<_>, _>>()?
        .chunks(args.max_args)
    {
        Command::new(&args.program)
            .args(&args.arguments)
            .args(lines)
            .spawn()?
            .wait()?;
    }

    Ok(())
}
