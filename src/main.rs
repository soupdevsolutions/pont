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
            let current_dir = std::env::current_dir()?;
            let current_dir_name = current_dir.clone().file_name().unwrap().to_str().unwrap().to_string();
            println!("Initializing a new Nemo project in the directory: {:?}", current_dir_name);
            
            let nemo_project = NemoProject::new(&current_dir_name, current_dir)?;
            nemo_project.save(false)?;
        }
        Some(("new", matches)) => {
            let name = matches.get_one::<String>("name").unwrap();
            let nemo_project = NemoProject::new(name, std::env::current_dir()?.join(name))?;
            println!("Creating a new Nemo project with the name: {}", name);
            nemo_project.save(true)?;
        }
        Some(("clone", matches)) => {
            let url = matches.get_one::<String>("url").unwrap();
            println!("Cloning a Nemo project from the url: {}", url);
        }
        _ => unreachable!(),
    }

    Ok(())
}
