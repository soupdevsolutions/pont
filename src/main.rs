use nemo::{
    file_management::Directory,
    nemo_project::{NemoProject, Source},
};
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
    let current_dir = Directory::current()?;
    match matches.subcommand() {
        Some(("init", _)) => {
            let nemo_project = NemoProject::try_from(&current_dir)?;
            nemo_project.save(false)?;
        }
        Some(("new", matches)) => {
            let name = matches.get_one::<String>("name").unwrap();
            let nemo_project = NemoProject::new(name, &current_dir)?;
            nemo_project.save(true)?;
        }
        Some(("build", matches)) => {
            let name = matches.get_one::<String>("name").unwrap();
            let source = matches.get_one::<String>("from").unwrap();

            let source: Url = source.parse()?;
            let source = Source::parse(&source)?;

            let target = current_dir.create_subdir(&name)?;

            let nemo_project = NemoProject::load(source, &target)?;

            nemo_project.build()?;
        }
        _ => unreachable!(),
    }

    Ok(())
}
