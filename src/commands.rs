use structopt::StructOpt;
use structopt::clap::arg_enum;

arg_enum! {
    #[derive(Debug)]
    pub enum EditMode {
        Latest,
        Interactive,
        Hash
    }
}

#[derive(Debug, StructOpt)]
pub enum DiaryCommand {
    #[structopt(about = "Re-scan the diary directory for some reason")]
    Scan,
    #[structopt(about = "Create a new Diary entry")]
    Create,
    #[structopt(about = "Edit a diary entry")]
    Edit {
        #[structopt(name = "edit mode", possible_values = &EditMode::variants(), default_value = "latest", case_insensitive = true)]
        mode: EditMode,
        hash: Option<String>
    },
    #[structopt(about = "List diary entries")]
    List,
    #[structopt(about = "Push un-synced diary entries to configured remotes")]
    Push
}



pub fn args() -> DiaryCommand {
    return DiaryCommand::from_args()
}
