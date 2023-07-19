fn main() {
   let cli = Cli::parse();

    match &cli.project_cmd {
        Some(cmd) => command::handle(&cmd),
        None => {}
    }
}
