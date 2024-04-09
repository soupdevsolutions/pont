use nemo::nemoproject::NemoProject;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let init_command = clap::Command::new("init");

    let new_command = clap::Command::new("new").arg(clap::Arg::new("name").required(true));

    let clone_command = clap::Command::new("clone").arg(clap::Arg::new("url").required(true));

    let commands = clap::Command::new("nemo")
        .subcommand_required(true)
        .subcommand(init_command)
        .subcommand(new_command)
        .subcommand(clone_command);

    let matches = commands.get_matches();
    match matches.subcommand() {
        Some(("init", _)) => {
            println!("Initializing a new Nemo project");
        }
        Some(("new", matches)) => {
            let name = matches.get_one::<String>("name").unwrap();
            let _nemo_project = NemoProject::create(name, std::env::current_dir()?.join(name))?;
            println!("Creating a new Nemo project with the name: {}", name);
        }
        Some(("clone", matches)) => {
            let url = matches.get_one::<String>("url").unwrap();
            println!("Cloning a Nemo project from the url: {}", url);
        }
        _ => unreachable!(),
    }

    Ok(())
}
