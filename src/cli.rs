use clap::{Parser, Subcommand};

use crate::project::handlers;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Option<Cmds>,
}

#[derive(Subcommand)]
pub enum Cmds {
    // Manage your projects
    Project {
        // Get logs from project + info
        id: Option<u32>,
        #[command(subcommand)]
        sub: ProjectSubCmds
    },
    // Manage logs
    Log {
        #[command(subcommand)]
        sub: LogSubCmds
    }
}

#[derive(Subcommand)]
pub enum ProjectSubCmds {
    // Create new project
    Add { name: String },
    // Remove project with the given ID
    Rm { id: u32 },
    // List projects
    Ls,
}

#[derive(Subcommand)]
pub enum LogSubCmds {
    // Start tracking time. Stops when SIGTERM is receieved
    Start {
        // Project to log time for
        project: String,
        // Optional description for work achieved
        message: Option<String>,
    },
    // Add a log to a project after the fact. Allows for manual duration input
    Add {
        // Project to log time for
        project: String,
        // Optional description for work achieved
        message: Option<String>,
        // Duration spent on log in minutes (m). Max value is 65535
        duration: u16,
    }
}

pub fn handle(cmd: &Cmds) {
    match cmd {
        Cmds::Project { id, sub } => {
            if let Some(id) = id {
                handlers::get(id)
            }

            match sub {
                ProjectSubCmds::Add { name } => handlers::add(name),
                ProjectSubCmds::Rm { id } => handlers::remove(id),
                ProjectSubCmds::Ls => handlers::list(),
            }
        },
        Cmds::Log { sub } => {
            match sub {
                LogSubCmds::Start { project, message } => println!("Log start called!"),
                LogSubCmds::Add { project, message, duration } => println!("Log add called!"),
            }
        }
    }
}
