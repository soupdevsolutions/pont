use pont::{
    file_management::Directory,
    pont_data::{PontProject, Source},
};

fn main() -> anyhow::Result<()> {
    let init_command = clap::Command::new("init");
    let new_command = clap::Command::new("new").arg(clap::arg!(--"name" <name>).required(true));
    let build_command = clap::Command::new("build")
        .arg(clap::arg!(--"name" <name>).required(true))
        .arg(clap::arg!(--"from" <from>).required(true));

    let commands = clap::Command::new("pont")
        .subcommand_required(true)
        .subcommand(init_command)
        .subcommand(new_command)
        .subcommand(build_command);

    let matches = commands.get_matches();
    let current_dir = Directory::current()?;
    match matches.subcommand() {
        Some(("init", _)) => {
            let pont_project = PontProject::from(&current_dir);
            pont_project.save()?;
        }
        Some(("new", matches)) => {
            let name = matches.get_one::<String>("name").unwrap();
            let directory = current_dir.create_subdir(name)?;
            let pont_project = PontProject::new(name, &directory);
            pont_project.save()?;
        }
        Some(("build", matches)) => {
            let name = matches.get_one::<String>("name").unwrap();
            let source = matches.get_one::<String>("from").unwrap();

            let source = Source::parse(source)?;
            let target = current_dir.create_subdir(name)?;

            let pont_project = PontProject::load(source, &target)?;

            pont_project.build()?;
        }
        _ => unreachable!(),
    }

    Ok(())
}
