use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};

pub(crate) fn create_app<'help>() -> App<'help> {
    App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .mut_arg("help", |help| {
            help.help("Display help information.").short('?')
        })
        .mut_arg("version", |v| v.help("Display version information."))
        .arg(
            Arg::new("WORD")
                .help("The word you want to search for")
                .required(false),
        )
        .arg(
            Arg::new("interactive")
                .help("Enter the interactive mode")
                .short('i')
                .long("interactive"),
        )
        .arg(
            Arg::new("save")
                .help("Save the words history to a log file ( words.log ).")
                .short('s')
                .long("save"),
        )
}
