use clap::{Parser, Subcommand};

#[derive(Clone, Debug, Subcommand)]
pub enum Opt {
    /// Start the wallpaper server at a specified directory
    Start {
        /// Directory containing wallpapers to cycle through
        directory: String,

        /// Redirect log output to log file
        #[arg(short = 'o', long)]
        log: Option<String>,

        /// Time (in seconds) between automatic wallpaper updates
        #[arg(short, long, default_value_t = 600)]
        duration: u64,

        /// Runs the wallpaper server in the current terminal (useful for debugging)
        #[arg(short, long = "foreground", default_value_t = false)]
        fg: bool,
    },

    /// Manually update the wallpaper with a provided path
    Update {
        /// Path to wallpaper
        path: String,
    },

    /// Cycle to the next wallpaper in the queue
    Next,

    /// Print out the current wallpaper directory
    GetDir,

    /// Set the directory to cycle through
    SetDir {
        /// Directory containing wallpapers to cycle through
        directory: String,
    },

    /// Ping the wallpaper server
    Ping,

    /// Stop the wallpaper server
    Kill,
}

/// A horribly written wallpaper engine with an unreasonably good name
#[derive(Debug, Parser, Clone)]
pub struct Args {
    #[command(subcommand)]
    pub command: Opt,
}
