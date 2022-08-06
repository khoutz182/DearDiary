use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct DDConfig {
    pub diary_directory: String,
    pub editor: Option<String>
}

impl std::default::Default for DDConfig {
    fn default() -> Self {
        Self {
            diary_directory: String::from("~/.diaries"),
            editor: Option::Some(String::from("nvim"))
        }
    }
}

impl DDConfig {

    pub fn expanded_diary_directory(&self) -> String {
        return shellexpand::tilde(&self.diary_directory).into();
    }
}
