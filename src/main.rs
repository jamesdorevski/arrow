use arrow::project;
use arrow::log;
use clap::{arg, Arg, ArgAction, Command, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    // Add, view or remove your projects
    Project {
        #[command(subcommand)]
        sub: ProjectSubCmds,
    }
}

#[derive(Subcommand)]
enum ProjectSubCmds {
    // Create new project
    Add { name: Option<String> },
    // List projects 
    Ls,
}

fn main() {
    println!("Hello, world!");
//
//    let matches = Command::new("Arrow")
//        .subcommand_required(true)
//        .arg_required_else_help(true)
//        .subcommand(
//            Command::new("project")
//                .about("Add, view or remove your projects")
//                .arg(
//                    Arg::new("list")
//                        .short('l')
//                        .long("list")
//                        .action(ArgAction::SetTrue)
//                        .help("list all created projects"),
//                )
//                .arg(
//                    Arg::new("add")
//                        .short('a')
//                        .long("add")
//                        .action(ArgAction::Set)
//                        .help("create a new project"),
//                )
//                .arg(
//                    Arg::new("rm")
//                        .short('r')
//                        .long("rm")
//                        .action(ArgAction::Set)
//                        .value_parser(clap::value_parser!(usize))
//                        .help("remove a project"),
//                )
//                .arg(arg!([name] "Get project logs")),
//        )
//        .subcommand(
//            Command::new("start")
//                .about("Start tracking time against a project. Use Ctrl+C to stop")
//                .arg(
//                    Arg::new("project")
//                        .short('p')
//                        .long("project")
//                        .action(ArgAction::Set)
//                        .help("Project to log time against"),
//                )
//                .arg(
//                    Arg::new("description")
//                        .short('d')
//                        .long("description")
//                        .action(ArgAction::Set)
//                        .help("Log description"),
//                ),
//        )
//        .get_matches();
//
//    match matches.subcommand() {
//        Some(("project", sub_matches)) => {
//            if sub_matches.get_flag("list") {
//                project::handlers::list();
//            }
//
//            if let Some(name) = sub_matches.get_one::<String>("add") {
//                project::handlers::add(name.to_string());
//            }
//
//            if let Some(id) = sub_matches.get_one::<usize>("rm") {
//                project::handlers::delete(id);
//            }
//
//            if let Some(name) = sub_matches.get_one::<String>("name") {
//                println!("Name entered!: {}", name);
//                project::handlers::get(3);
//            }
//        }
//        Some(("start", sub_matches)) => {
//            // get project using project id
//            log::handlers::start_logging("DEBUG".to_string());
//            // if it doesn't exist, create it
//        }
//        _ => unreachable!("Exhausted list of subcommands"),
//    }
    let cli = Cli::parse();  

    match &cli.command {
        Some(Commands::Project { sub }) => {
        },
        None => {},
    }
}
