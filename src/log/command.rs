use clap::Subcommand;

#[derive(Subcommand)]
enum LogCmds {
    // Manage logs
    Log {
        #[command(subcommand)]
        sub: LogSubCmds,
    },
}

#[derive(Subcommand)]
enum LogSubCmds {
    // Start tracking time. Stops when SIGTERM is recieved
    Start {
        // Project to log time for
        project: String,
        // Optional description for work achieved
        message: Option<String>
    },
    // Add a log to a project after the fact. Allows you to input taken
    Add {
        // Project to log time for
        project: String,
        // Optional description for work achieved
        message: Option<String>,
        // Duration spent on log in seconds (s) Max value is 65535
        duration: u16
    }
}
