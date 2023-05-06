mod capture;
mod extract;
mod merge;
mod save;

pub use extract::Extract;
pub use merge::Merge;
pub use save::{Save, Quality};

#[cfg(feature = "browser")]
pub use capture::Capture;

use clap::{Parser, Subcommand};

/// Download video streams served over HTTP from websites, HLS and DASH playlists.
#[derive(Debug, Clone, Parser)]
#[command(version, author = "clitic <clitic21@gmail.com>", about)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Commands {
    #[cfg(feature = "browser")]
    Capture(Capture),
    Extract(Extract),
    Merge(Merge),
    Save(Save),
}
