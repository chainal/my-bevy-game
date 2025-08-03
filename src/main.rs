use clap::{Parser, Subcommand};

mod rotate;

#[derive(Parser)]
struct Args {
    #[clap(subcommand)]
    cmd: Cmd
}

#[derive(Subcommand)]
enum Cmd {
    Hello,
    BevyRotate
}

fn main() {
    let args = Args::parse();
    match args.cmd {
        Cmd::Hello => println!("Hello, world!"),
        Cmd::BevyRotate => rotate::entro(),
    }
}
