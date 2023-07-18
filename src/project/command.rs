use clap::Subcommand;

#[derive(Subcommand)]
pub enum ProjectCmds {
    // Add, view or remove your projects
    Project {
        // Get logs from project + info
        name: Option<i64>,
        #[command(subcommand)]
        sub: ProjectSubCmds,
    }
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


