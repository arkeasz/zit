pub mod app;
use std::fs;
use std::env;
use std::os::windows::fs::MetadataExt;
use chrono::{DateTime, Local};
use crossterm::style::Stylize;
use app::{App, Entry};


fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let file: &String = if args.len() == 1 { &".".to_string() } else {&args[1]};
    let app: App= App::init(file);
    for entry in app.entries   {
        println!(
            "{:<6} \t {:<19} {:>8} {}",
            entry.mode, entry.last_modified, entry.lenght, entry.name
        );
    }

    Ok(())
}
