use clap::Subcommand;

pub mod logs;

#[derive(Subcommand)]
pub enum Cmd {
    Logs,
}