use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

/// A fast, file-based CLI note-taking application using .note files
#[derive(Parser, Debug)]
#[command(name = "rusty-notes")]
#[command(version)]
#[command(about, long_about = None)]
pub struct Cli {
    /// Notes directory
    #[arg(short, long, global = true, default_value = "~/.notes")]
    pub dir: String,

    /// Editor to open notes with
    #[arg(short, long, global = true, env = "EDITOR")]
    pub editor: Option<String>,

    /// Enable verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Suppress output
    #[arg(short, long, global = true)]
    pub quiet: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Create a new note
    New {
        /// Title of the note
        title: Option<String>,

        /// Add tags
        #[arg(short, long)]
        tag: Vec<String>,

        /// Use a template
        #[arg(short = 'T', long)]
        template: Option<String>,

        /// Create note in inbox
        #[arg(short, long)]
        inbox: bool,

        /// Set creation date (YYYY-MM-DD)
        #[arg(short = 'D', long)]
        date: Option<String>,

        /// Do not open editor after creation
        #[arg(long)]
        no_open: bool,
    },

    /// Open an existing note
    Open {
        /// Note identifier or search query
        query: String,

        /// Override editor
        #[arg(short, long)]
        editor: Option<String>,

        /// Open in read-only mode
        #[arg(short, long)]
        readonly: bool,
    },

    /// List notes
    List {
        /// Filter by tag
        #[arg(short, long)]
        tag: Vec<String>,

        /// Filter by folder
        #[arg(short, long)]
        folder: Option<PathBuf>,

        /// Sort by field
        #[arg(short, long, value_enum)]
        sort: Option<SortField>,

        /// Reverse sort order
        #[arg(short, long)]
        reverse: bool,

        /// Show today's notes
        #[arg(long)]
        today: bool,

        /// Output as JSON
        #[arg(long)]
        json: bool,
    },

    /// Search notes by content
    Search {
        /// Search query
        query: String,

        /// Filter by tag
        #[arg(short, long)]
        tag: Vec<String>,

        /// Case-insensitive search
        #[arg(short, long)]
        ignore_case: bool,

        /// Context lines
        #[arg(short, long, default_value_t = 2)]
        context: usize,

        /// Search only titles
        #[arg(long)]
        titles_only: bool,

        /// Print only file paths
        #[arg(long)]
        paths_only: bool,
    },

    /// Manage tags
    Tag {
        #[command(subcommand)]
        command: TagCommands,
    },

    /// Move notes between folders
    Move {
        /// Note identifier
        note: String,

        /// Destination folder
        destination: PathBuf,

        /// Create folder if missing
        #[arg(short, long)]
        create: bool,
    },

    /// Archive notes
    Archive {
        /// Note identifier
        note: String,

        /// Archive all matching notes
        #[arg(long)]
        all: bool,
    },

    /// Delete notes
    Delete {
        /// Note identifier
        note: String,

        /// Skip confirmation
        #[arg(short, long)]
        force: bool,
    },

    /// Manage configuration
    Config {
        #[command(subcommand)]
        command: ConfigCommands,
    },

    /// Check setup and environment
    Doctor,
}

#[derive(Subcommand, Debug)]
pub enum TagCommands {
    /// Add tags to a note
    Add {
        note: String,
        tags: Vec<String>,
    },

    /// Remove tags from a note
    Remove {
        note: String,
        tags: Vec<String>,
    },

    /// Rename a tag
    Rename {
        old: String,
        new: String,
    },

    /// List all tags
    List,
}

#[derive(Subcommand, Debug)]
pub enum ConfigCommands {
    /// Get a config value
    Get { key: String },

    /// Set a config value
    Set { key: String, value: String },

    /// List all config values
    List,
}

#[derive(ValueEnum, Debug, Clone)]
pub enum SortField {
    Date,
    Title,
    Updated,
}
