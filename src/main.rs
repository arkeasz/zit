pub mod app;
pub mod ref_command;
pub mod parser;
use std::{env, fmt::format};
use app::App;
use ref_command::*;


fn main() -> std::io::Result<()> {
    let app: App = App::init();


    for dir in &app.dirs {
        println!("{}", dir);
        for entry in &app.entries    {
            if dir == &entry.father  {
                println!(
                    "{:<6} \t {:<19} {:>8} {}",
                    entry.mode, entry.last_modified, entry.lenght, entry.name
                );
            }

        }
        println!("")
    }

    Ok(())
}
