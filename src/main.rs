use crate::commands::{DiaryCommand, EditMode};
use crate::config::DDConfig;
use chrono::prelude::*;
use std::{env, io, fs};
use std::io::Write;
use std::process::Command;
use std::str;
use uuid::Uuid;

mod commands;
mod config;

struct Context {
    config: DDConfig,
}

fn main() -> Result<(), io::Error> {
    let cmd = commands::args();
    let cfg: config::DDConfig = confy::load("dear-diary")?;
    let context = Context { config: cfg };

    // Create diary directory if it doesn't exist
    fs::create_dir_all(context.config.expanded_diary_directory())?;

    match cmd {
        DiaryCommand::Scan => scan(),
        DiaryCommand::List => list(context),
        DiaryCommand::Create => create(&context),
        DiaryCommand::Edit { mode, hash } => match mode {
            EditMode::Latest => edit_entry(latest_entry()),
            EditMode::Interactive => edit_entry(interactive()),
            EditMode::Hash => match hash {
                Some(h) => edit_entry(h),
                None => panic!("oh no"),
            },
        },
        DiaryCommand::Push => push(),
    }

    return Ok(());
}

// TODO: make generic
fn read_input(prompt: &str) -> String {
    print!("{}", prompt);

    match io::stdout().flush() {
        Err(e) => eprintln!("Error flushing stdout: {}", e),
        _ => {}
    }

    let mut input_val = String::new();
    let stdin = io::stdin();
    match stdin.read_line(&mut input_val) {
        Ok(_) => {}
        Err(e) => panic!("{}", e),
    }
    input_val.pop();
    return input_val.replace(" ", "_");
}

fn get_editor(config: &DDConfig) -> Result<String, String> {
    if !config.editor.is_some() {
        let editor = config.editor.clone();
        return Ok(editor.unwrap_or_default());
    }

    let env_variables = ["DIARY_EDITOR", "EDITOR"];
    for editor in env_variables.iter() {
        let env_editor = env::var(*editor);
        match env_editor {
            Ok(editor) => return Ok(editor),
            _ => {}
        }
    }
    return Err(String::from(
        "Editor not set in configuration or defined in environment variables",
    ));
}

fn scan() {
    println!("scanning diary entries")
}

fn create(context: &Context) {
    let path_sep = std::path::MAIN_SEPARATOR;
    let edit_cmd_cfg = get_editor(&context.config);
    let edit_cmd = match edit_cmd_cfg {
        Ok(cmd) => cmd,
        Err(e) => panic!("{:?}", e),
    };
    let date_format = format!("%Y{}%m{}%d", path_sep, path_sep);
    let date_path = Local::now().format(date_format.as_str());
    let title = read_input("Enter title of diary: ");
    let uuid = Uuid::new_v4();

    let diary_fullpath = format!(
        "{}{}{}{}{}-{}.md",
        context.config.expanded_diary_directory(), path_sep, date_path, path_sep, uuid, title
    );

    let output = Command::new(edit_cmd)
        .arg(diary_fullpath)
        .output()
        .expect("Failed to execute process");

    println!("output status: {}", output.status);
    let stdout = str::from_utf8(&output.stdout);
    let stderr = str::from_utf8(&output.stderr);
    println!("out/err: {:?} | {:?}", stdout, stderr);
}

fn list(context: Context) {
    match fs::read_dir(context.config.expanded_diary_directory()) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(paths) => for path in paths {
            println!("> {:?}", path.unwrap().path());
        }
    }
}

fn interactive() -> String {
    return String::from("interactive");
}

fn latest_entry() -> String {
    return String::from("latest");
}

fn edit_entry(entry_hash: String) {
    println!("editing entry: {entry}", entry = entry_hash);
}

fn push() {
    println!("pushing diary entries")
}
