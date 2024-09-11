use std::fs;
use std::env;
use std::os::windows::fs::MetadataExt;
use chrono::{DateTime, Local};
use crossterm::style::Stylize;

#[derive(Debug)]
pub struct App {
    pub entries: Vec<Entry>
}

#[derive(Debug)]
pub struct Entry    {
    pub mode: String,
    pub last_modified: String,
    pub name: String,
    pub lenght: u64
}
impl Entry  {
    pub fn new() -> Entry {
        Entry {
            mode: String::new(),
            last_modified: String::new(),
            name: String::new(),
            lenght: 0
        }
    }
}
impl App    {
    pub fn init(file: &String) -> App    {
        let mut entries : Vec<Entry> = Vec::new();
        if let Ok(entriez) = fs::read_dir::<&String>(file)  {
            for entry in entriez    {
                let mut entry_dir = Entry::new();
                if let Ok(entry) = entry    {
                    // entry_dir.name = format!("{}", entry.file_name());
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
                    match entry.file_name().to_str() {
                        Some(s) => {
                            entry_dir.name = s.to_string()
                        }
                        None => {
                            eprintln!("Failed to convert OsString to String due to invalid UTF-8");
                        }
                    }
                }
                entries.push(entry_dir)
            }
        } else {
            println!("the file couldnt be found")
        }
        App { entries }
    }
}
