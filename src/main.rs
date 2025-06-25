use anyhow::anyhow;
use std::env::args;
use std::error::Error;
use std::io::stdin;
use std::process::Command;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = args().skip(1).collect();
    for line in stdin().lines() {
        Command::new(args.first().ok_or(anyhow!("Missing program name"))?)
            .args(args.iter().skip(1))
            .arg(line?)
            .spawn()?
            .wait()?;
    }

    Ok(())
}
