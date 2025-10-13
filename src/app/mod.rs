use clap::Subcommand;

pub mod logs;
pub mod no_renderer;
mod plugin;

pub use plugin::entro as plugin;

#[derive(Subcommand)]
pub enum Cmd {
    Logs,
    NoRenderer,
    Plugin,
}