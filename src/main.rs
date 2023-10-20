//引入lib
use clap::{Arg, ArgAction, Command};
use rtd::Item;

#[allow(unused)]
fn main() {
    let mut todo_item = Item::new();
    let matches = Command::new("rtd")
        .about(" A command line tool for todo things")
        .version("0.1.0")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("add")
                .about("Add a new todo 👍")
                .short_flag('a')
                .long_flag("add")
                .arg(
                    Arg::new("todo")
                        .help("The todo to add")
                        .action(ArgAction::Set),
                ),
        )
        .subcommand(
            Command::new("list")
                .about("List all todos 📋")
                .short_flag('l')
                .long_flag("list"),
        )
        .subcommand(
            Command::new("done")
                .about("Mark a todo as done ✅")
                .short_flag('d')
                .long_flag("done")
                .arg(
                    Arg::new("no")
                        .help("The todo to mark as done")
                        .action(ArgAction::Set),
                ),
        )
        .subcommand(
            Command::new("clear")
                .about("Clear all todos 🗑")
                .short_flag('c')
                .long_flag("clear"),
        )
        .get_matches();
    match matches.subcommand() {
        Some(("add", add_matches)) => {
            let todo = add_matches.try_get_raw("todo").expect("Failed to get todo");
            if let Some(todo) = todo {
                todo.into_iter().for_each(|s| {
                    println!("Adding todo: {:?}", s);
                    //需要转化为String
                    let s = s.to_str().unwrap();
                    todo_item.add_item(s.to_string());
                });
            } else {
                println!("No todo provided");
            }
        }
        Some(("list", _)) => {
            println!("Listing todos");
            todo_item.list_items();
        }
        Some(("done", done_matches)) => {
            println!("Marking todo as done");
            let no = done_matches.try_get_raw("no").expect("Failed to get no");
            if let Some(no) = no {
                no.into_iter().for_each(|s| {
                    println!("Marking todo as done: {:?}", s);
                    let s = s.to_str().unwrap();
                    let no = s.parse::<usize>().unwrap();
                    todo_item.done_items(no);
                });
            } else {
                println!("No todo provided");
            }
        }
        Some(("clear", _)) => {
            println!("Clearing all todos");
            todo_item.clear_items();
        }
        _ => unreachable!(),
    }
}
