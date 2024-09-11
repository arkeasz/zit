use std::fs;
use std::env;
use std::os::windows::fs::MetadataExt;
use chrono::{DateTime, Local};
use crossterm::style::Stylize;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    println!("{}", args.len());
    let file = if args.len() == 1 { &".".to_string() } else {&args[1]};
    if let Ok(entries) = fs::read_dir::<&String>(file)  {
        for entry in entries    {
           if let Ok(entry) = entry    {
            println!();
                if let Ok(metadata) = fs::metadata(entry.path())    {
                    // let size = metadata.len();
                    let file_size = metadata.file_size();
                    let permissions = metadata.permissions();
                    let perm_str = format!(
                        "{}{}{}{}",
                        if metadata.is_dir() { "d" } else {"-"},
                        if metadata.is_file() { "a" } else {"-"},
                        if permissions.readonly() { "r" } else { "-" },
                        "-"
                    );
                    print!(
                        "{:>9}",
                        perm_str.blue()
                    );
                    if let Ok(modified_time) = metadata.modified()  {
                        let datetime: DateTime<Local> = modified_time.into();
                        print!("\t{:>12}", datetime.format("%d/%m/%Y %H:%M"))
                    } else {
                        println!("Couldn't retrieve the last modified time")
                    }
                }
                if let Ok(file_type) = entry.file_type()    {
                    println!("{:?}: {:?}", entry.file_name(), file_type)
                } else {
                    println!("Couldnt get file type for {:?}", entry.path())
                }
            }
        }
    } else {
        println!("the file couldnt be found")
    }

    Ok(())
}
