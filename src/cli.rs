use clap::{Parser, Subcommand};

use crate::project::handlers;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    cmd: Option<Cmds>,
}

#[derive(Subcommand)]
pub enum Cmds {
    // Manage your projects
    Project {
        // Get logs from project + info
        id: Option<u32>,
        #[command(subcommand)]
        sub: ProjectSubCmds
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
        }
    }
}
