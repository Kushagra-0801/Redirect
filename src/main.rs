use std::{error::Error, fs::File, path::PathBuf, process::Command};

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// The command to run
    command: PathBuf,
    /// The arguments to pass to the command
    args: Vec<String>,
    #[clap(short, long)]
    /// File to read stdin from - default is from stdin
    input: Option<PathBuf>,
    /// File to write stdout to - default is to stdout
    #[clap(short, long)]
    output: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let input_file;
    let output_file;
    let mut cmd = Command::new(args.command);
    cmd.args(args.args);
    if let Some(path) = args.input {
        input_file = File::open(path)?;
        cmd.stdin(input_file);
    }
    if let Some(path) = args.output {
        output_file = File::create(path)?;
        cmd.stdout(output_file);
    }
    cmd.status()?
        .success()
        .then_some(())
        .ok_or_else(|| "Command failed to complete".into())
}
