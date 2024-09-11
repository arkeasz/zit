use std::fs;
use std::env;
use std::os::windows::fs::MetadataExt;
use chrono::{DateTime, Local};
use crossterm::style::Stylize;

// struct Entries {
//     entries: Vec<Entry>
// }
// struct Entry    {
//     pub mode: String,
//     pub last_modified: DateTime<Local>,
//     pub name: String,
//     pub lenght: u64
// }

// impl Entry  {
//     fn new() -> Entry {
//         Entry {
//             mode: String::new(),
//             last_modified: Local::now(),
//             name: String::new(),
//             lenght: 0
//         }
//     }
// }
// impl Entries    {
//     fn init(file: &String) -> Entries    {
//         let mut entries : Vec<Entry> = Vec::new();
//         if let Ok(entries) = fs::read_dir::<&String>(file)  {
//             for entry in entries    {
//                 let mut entry_dir = Entry::new();
//                 if let Ok(entry) = entry    {
//                     // entry_dir.name = format!("{}", entry.file_name());
//                     if let Ok(metadata) = fs::metadata(entry.path())    {
//                         entry_dir.lenght = metadata.file_size();
//                         let permissions = metadata.permissions();
//                         entry_dir.mode = format!(
//                             "{}{}{}{}",
//                             if metadata.is_dir() { "d" } else {"-"},
//                             if metadata.is_file() { "a" } else {"-"},
//                             if permissions.readonly() { "r" } else { "-" },
//                             "-"
//                         );
//                         if let Ok(modified_time) = metadata.modified()  {
//                             entry_dir.last_modified = modified_time.into();
//                             // println!("Last modified: {}", datetime.format("%d/%m/%Y %H:%M:%S"))
//                         } else {
//                             println!("Couldn't retrieve the last modified time")
//                         }
//                     }
//                     if let Ok(file_type) = entry.file_type()    {
//                         println!("{:?}: {:?}", entry.file_name(), file_type)
//                     } else {
//                         println!("Couldnt get file type for {:?}", entry.path())
//                     }
//                 }

//             }
//         } else {
//             println!("the file couldnt be found")
//         }

//         Entries { entries }
//     }
// }

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
