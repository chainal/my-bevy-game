use clap::{Parser, Subcommand};

mod rotate;
mod transparency_2d;
mod app;

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
    App {
        #[clap(subcommand)]
        cmd: app::Cmd
    },
}

fn main() {
    let args = Args::parse();
    match args.cmd {
        Cmd::Hello => println!("Hello, world!"),
        Cmd::BevyRotate => rotate::entro(),
        Cmd::Transparency2d => transparency_2d::entro(),
        Cmd::App { cmd: app::Cmd::Logs } => app::logs::entro(),
    }
}
