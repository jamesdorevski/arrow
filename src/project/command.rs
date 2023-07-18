use clap::Subcommand;

use super::handlers;

#[derive(Subcommand)]
pub enum ProjectCmds {
    // Add, view or remove your projects
    Project {
        // Get logs from project + info
        id: Option<i64>,
        #[command(subcommand)]
        sub: ProjectSubCmds,
    },
}

#[derive(Subcommand)]
pub enum ProjectSubCmds {
    // Create new project
    Add { name: String },
    // Remove project with the given ID
    Rm { id: i64 },
    // List projects
    Ls,
}

pub fn handle(cmd: &ProjectCmds) {
    match cmd {
        ProjectCmds::Project { id, sub } => {
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
