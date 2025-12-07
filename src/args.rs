use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version, author, about, long_about = None)]
pub struct Args {
    /// Set log level
    #[clap(long, short, default_value = "info")]
    pub log_level: crate::config::LogLevel,

    /// Path to config file, overrides default
    #[clap(long, short)]
    pub config: Option<String>,

    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Parser)]
pub enum Command {
    /// Publish default config to $BLEND_HOME or the OS-default config directory
    Publish {
        /// Overwrite existing config file if it already exists
        #[clap(long, short)]
        force: bool,
    },

    /// Start web and worker processes
    Start,
}
