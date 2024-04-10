use nemo::nemoproject::{NemoProject, Source};
use url::Url;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let init_command = clap::Command::new("init");
    let new_command = clap::Command::new("new").arg(clap::arg!(--"name" <name>).required(true));
    let build_command = clap::Command::new("build")
        .arg(clap::arg!(--"name" <name>).required(true))
        .arg(clap::arg!(--"from" <from>).required(true));

    let commands = clap::Command::new("nemo")
        .subcommand_required(true)
        .subcommand(init_command)
        .subcommand(new_command)
        .subcommand(build_command);

    let matches = commands.get_matches();
    match matches.subcommand() {
        Some(("init", _)) => {
            let current_dir = std::env::current_dir()?;
            let current_dir_name = current_dir
                .clone()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            let nemo_project = NemoProject::new(&current_dir_name, current_dir)?;

            println!(
                "Initializing a new Nemo project in the directory: {:?}",
                current_dir_name
            );
            nemo_project.save(false)?;
        }
        Some(("new", matches)) => {
            let name = matches.get_one::<String>("name").unwrap();
            let nemo_project = NemoProject::new(name, std::env::current_dir()?.join(name))?;

            println!("Creating a new Nemo project with the name: {}", name);
            nemo_project.save(true)?;
        }
        Some(("build", matches)) => {
            let name = matches.get_one::<String>("name").unwrap();
            let source = matches.get_one::<String>("from").unwrap();
            let source: Url = source.parse()?;
            let source = Source::parse(&source)?;

            let dir_name = std::env::current_dir()?.join(name);

            println!("Building Nemo project {} from: {:?}", name, source);
            let nemo_project = NemoProject::load(source, &dir_name)?;
            println!("Loaded Nemo project: {:?}", nemo_project);
        }
        _ => unreachable!(),
    }

    Ok(())
}
