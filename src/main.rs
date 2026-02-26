mod archive;
mod checkpoint;
mod init;
mod nudge;
mod parser;
mod paths;
mod pulse;
mod review;
mod scan;
mod state;
mod status;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "praxis-echo",
    about = "Pipeline enforcement for AI self-evolution",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize the pipeline enforcement system
    Init,
    /// Inject pipeline state at session start (PreToolUse hook)
    Pulse,
    /// Snapshot document state before context loss (PreCompact hook)
    Checkpoint,
    /// Post-session pipeline review (SessionEnd hook)
    Review,
    /// Deep scan of all documents
    Scan {
        /// Output format: human or json
        #[arg(long, default_value = "human")]
        format: String,
    },
    /// Enforce thresholds — archive overflow content
    Archive {
        /// Show what would be archived without doing it
        #[arg(long)]
        dry_run: bool,
    },
    /// Queue a curiosity-driven intent into the self-schedule system
    Nudge {
        /// The curiosity question or topic to explore
        #[arg(long)]
        topic: String,
        /// When to schedule (ISO 8601 or relative like "+2h", "+30m")
        #[arg(long, default_value = "+2h")]
        when: String,
        /// Priority: low, normal, high
        #[arg(long, default_value = "normal")]
        priority: String,
    },
    /// Pipeline health dashboard
    Status,
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Some(Commands::Init) | None => init::run(),
        Some(Commands::Pulse) => pulse::run(),
        Some(Commands::Checkpoint) => checkpoint::run(),
        Some(Commands::Review) => review::run(),
        Some(Commands::Scan { format }) => scan::run(&format),
        Some(Commands::Archive { dry_run }) => archive::run(dry_run),
        Some(Commands::Nudge {
            topic,
            when,
            priority,
        }) => nudge::run(&topic, &when, &priority),
        Some(Commands::Status) => status::run(),
    };

    if let Err(e) = result {
        eprintln!("\x1b[31m✗\x1b[0m {e}");
        std::process::exit(1);
    }
}
