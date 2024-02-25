use clap::{Parser, Subcommand};

use crate::project;
//use crate::log;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Option<Cmds>,
}

#[derive(Subcommand)]
pub enum Cmds {
    /// Manage your projects
    Project {
        /// Get logs from project + info
        id: Option<u32>,
        #[command(subcommand)]
        sub: ProjectSubCmds
    },
    // // Manage logs
    // Log {
    //     #[command(subcommand)]
    //     sub: LogSubCmds
    // }
}

#[derive(Subcommand)]
pub enum ProjectSubCmds {
    /// Create new project to track time spent working towards a work item, task, or goal.
    New { 
        /// Name of the project
        name: String,
        /// Optional project description
        description: Option<String>
    },
    // // Remove project with the given ID
    // Rm { id: u32 },
    /// List projects
    Ls,
    /// Edit a project
    Edit {
        /// ID of the project to edit
        id: u32,
        /// New name for project
        #[arg(short, long)]
        name: Option<String>,
        /// New description for project
        #[arg(short, long)]
        description: Option<String>,
    }
}

#[derive(Subcommand)]
pub enum LogSubCmds {
    // Start tracking time. Stops when SIGTERM is received
    Start {
        // Project to log time for
        project: u32,
        // Optional description for work achieved
        message: Option<String>,
    },
    // Add a log to a project after the fact. Allows for manual duration input
    Add {
        // Project to log time for
        project: u32,
        // Optional description for work achieved
        message: Option<String>,
        // Duration spent on log in minutes (m). Max value is 65535
        duration: u16,
    },
    // Remove log from a project
    // Rm {
    //     // Project to remove log for
    //     project: u32,
    //     // Id of log to remove
    //     log: u32,
    // }
}

pub fn handle(cmd: &Cmds) {
    match cmd {
        Cmds::Project { id, sub } => {
            if let Some(id) = id {
                // project::handlers::get(id)
            }

            match sub {
                ProjectSubCmds::New { name, description } => project::handlers::new(name.to_owned(), description.to_owned()),
                // ProjectSubCmds::Rm { id } => project::handlers::remove(id),
                ProjectSubCmds::Ls => project::handlers::list(),
                ProjectSubCmds::Edit { id, name, description } => project::handlers::edit(*id, name.to_owned(), description.to_owned()),
            }
        },
        // Cmds::Log { sub } => {
        //     match sub {
        //         LogSubCmds::Start { project, message } => log::handlers::start_logging(project, message.clone()),
        //         LogSubCmds::Add { project, message, duration } => log::handlers::save_log(project, message.clone(), duration),
        //         LogSubCmds::Rm { project, log } => log::handlers::remove_log(project, log),
        //     };
        // }
    }
}
