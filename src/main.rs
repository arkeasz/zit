pub mod app;
pub mod ref_command;
pub mod parser;
pub mod window;
use app::App;
use parser::Opti;
use window::get_terminal_size;

fn main() {
    if let Some(app) = App::init() {
        let (term_width, _) = get_terminal_size();

        let mut column_widths = vec![];
        let items = app.entries;
        for item in &items{
            column_widths.push(item.name.len());
        }

        let max_item_len = *column_widths.iter().max().unwrap_or(&0) + 2;
        let columns = term_width as usize / max_item_len;

        if app.dirs.is_empty()  {
            for (i, entry) in items.iter().enumerate()   {

                if app.options.contains(&Opti::List)  {
                    println!(
                        "{:<6} \t {:<19} {:>8} {}",
                        entry.mode, entry.last_modified, entry.lenght, entry.name
                    );
                } else {
                    print!("{:width$}", entry.name, width = max_item_len);

                    if (i + 1) % columns == 0 {
                        println!();
                    }
                }
            }

            if items.len() % columns != 0 {
                println!();
            }
        } else {
            for dir in &app.dirs {
                for (i, entry) in items.iter().enumerate()    {
                    if dir == &entry.father  {
                        if app.options.contains(&Opti::List)  {
                            println!(
                                "{:<6} \t {:<19} {:>8} {}",
                                entry.mode, entry.last_modified, entry.lenght, entry.name
                            );
                        } else {
                            print!("{:width$}", entry.name, width = max_item_len);

                            if (i + 1) % columns == 0 {
                                println!();
                            }

                        }
                    }

                }
            }
            if items.len() % columns != 0 {
                println!();
            }
        }
    } else {
        return;
    }
}
