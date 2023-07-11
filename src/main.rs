use clap::{Command, Arg, ArgAction};

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
                        .help("list all created projects")
                )
                .arg(
                    Arg::new("add")
                        .short('a')
                        .long("add")
                )
        )
        .get_matches();
    
    match matches.subcommand() {
        Some(("project", sub_matches)) => {
            println!("project list used.");
            
            let list_selected = sub_matches.get_flag("list");
            println!("{}", list_selected);
        },
        _ => unreachable!("Exhausted list of subcommands and "),
    }
}
