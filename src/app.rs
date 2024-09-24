use std::fs;
use std::ops::Index;
use std::os::windows::fs::MetadataExt;
use chrono::{DateTime, Local};
use crossterm::style::Stylize;
use crate::parser::{parse, Option};
use crate::ref_command::*;

#[derive(Clone)]
pub struct App {
    pub entries: Vec<Entry>,
    pub dirs: Vec<String>,
    pub name: &'static str,
    pub version: &'static str,
    pub options: Vec<Option>

}
#[derive(Clone)]
pub struct Entry    {
    pub mode: String,
    pub last_modified: String,
    pub name: String,
    pub lenght: u64,
    pub father: String,
}
impl Entry  {
    pub fn new() -> Entry {
        Entry {
            mode: String::new(),
            last_modified: String::new(),
            name: String::new(),
            lenght: 0,
            father: String::new(),
        }
    }
}
impl App    {
    pub fn init() -> App    {
        let mut app = App {
            entries: Vec::new(),
            dirs: Vec::new(),
            name: &NAME,
            version: &VERSION,
            options: Vec::new()
        };
        let (options, values) = parse();
        let mut entries: Vec<Entry> = Vec::new();
        let mut dirs: Vec<String> = Vec::new();
        for op in options   {
            match op.as_str()    {
                "--help" | "-h" => { help(); break},
                "--version" | "-v" => { version(); break},
                "--all" | "-a" => app.options.push(Option::All),
                "--list" | "-l" => app.options.push(Option::List),
                _ => todo!()
            }
        }


        for val in values   {

            if let Ok(dir) = fs::read_dir::<&String>(&val)   {
                dirs.push(val.clone());
                for entry in dir    {
                    let mut entry_dir = Entry::new();
                    if let Ok(entry) = entry    {
                        entry_dir.father = val.clone();
                        if let Some(filename) = entry.file_name().to_str()  {
                            if filename.starts_with('.') && app.options.contains(&Option::All) {
                                continue;
                            }
                            if let Ok(metadata) = fs::metadata(entry.path())    {
                                entry_dir.lenght = metadata.file_size();
                                let permissions = metadata.permissions();
                                entry_dir.mode = format!(
                                    "{}{}{}{}",
                                    if metadata.is_dir() { "d" } else {"-"},
                                    if metadata.is_file() { "a" } else {"-"},
                                    if permissions.readonly() { "r" } else { "-" },
                                    "-"
                                );
                                if let Ok(modified_time) = metadata.modified()  {
                                    let datetime: DateTime<Local> = modified_time.into();
                                    entry_dir.last_modified = datetime.format("%d/%m/%Y\t%H:%M").to_string()
                                } else {
                                    println!("Couldn't retrieve the last modified time")
                                }
                            }
                            entry_dir.name = filename.to_string();
                        }


                        // match entry.file_name().to_str() {
                        //     Some(s) => {
                        //         entry_dir.name = s.to_string();
                        //     }
                        //     None => {
                        //         eprintln!("Failed to convert OsString to String due to invalid UTF-8");
                        //     }
                        // }
                    }
                    entries.push(entry_dir)
                }
           } else {
               continue;
           };
        }

        app.entries = entries;
        app.dirs = dirs;
        app

    }
}
