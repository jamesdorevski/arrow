use clap::{Arg, ArgAction, Command};

fn main() {
    println!("Hello, world!");

    let matches = Command::new("Arrow")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("project")
                .about("Add, view or remove your projects")
                .arg(
                    Arg::new("list")
                        .short('l')
                        .long("list")
                        .action(ArgAction::SetTrue)
                        .help("list all created projects"),
                )
                .arg(
                    Arg::new("add")
                        .short('a')
                        .long("add")
                        .action(ArgAction::Set)
                        .help("create a new project"),
                )
                .arg(
                    Arg::new("rm")
                        .short('r')
                        .long("rm")
                        .action(ArgAction::Set)
                        .value_parser(clap::value_parser!(usize))
                        .help("remove a project"),
                )
        )
        .subcommand(
            Command::new("start")
                .about("Start tracking time against a project. Use Ctrl+C to stop")
                .arg(
                    Arg::new("project")
                        .short('p')
                        .long("project")
                        .action(ArgAction::Set)
                        .help("Project to log time against")
                )
        )
        .get_matches();

    match matches.subcommand() {
        Some(("project", sub_matches)) => {
            if sub_matches.get_flag("list") {
                project::list();
            }

            if let Some(name) = sub_matches.get_one::<String>("add") {
                project::add(name.to_string());
            }

            if let Some(id) = sub_matches.get_one::<usize>("rm") {
                project::delete(id);
            }

        }
        _ => unreachable!("Exhausted list of subcommands"),
    }
}
