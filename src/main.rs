use clap::{Parser, Subcommand};

mod rotate;
mod transparency_2d;

#[derive(Parser)]
struct Args {
    #[clap(subcommand)]
    cmd: Cmd
}

#[derive(Subcommand)]
enum Cmd {
    Hello,
    BevyRotate,
    Transparency2d,
}

fn main() {
    let args = Args::parse();
    match args.cmd {
        Cmd::Hello => println!("Hello, world!"),
        Cmd::BevyRotate => rotate::entro(),
        Cmd::Transparency2d => transparency_2d::entro(),
    }
}
