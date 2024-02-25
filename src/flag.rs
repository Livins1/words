use clap::ArgMatches;

#[derive(Default, Clone, Copy)]
pub(crate) struct Flags {
    pub save: bool,
    pub interactive: bool,
}

impl Flags {
    pub fn from_matchs(matches: &ArgMatches) -> Self {
        let save = matches.is_present("save");
        let interactive = matches.is_present("interactive");
        Flags { save, interactive }
    }
}
