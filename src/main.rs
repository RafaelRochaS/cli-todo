use chrono::Utc;
use clap::{App, Arg, SubCommand};
use cli_todo::models::task;

fn main() {
    let task = task::Task {
        id: 200,
        description: "test".to_string(),
        completed: true,
        created_at: Utc::now().to_string(),
        update_at: Utc::now().to_string(),
    };
    println!("Basic task: {:?}", task);

    let app = create_app();
    let matches = app.get_matches();
    println!("Matches: {:?}", matches);
}

fn create_app() -> App<'static> {
    return App::new("Todo List")
        .version("1.0")
        .author("Joe das Tretas <joe@tretas.com>")
        .about("Manages a todo list")
        .subcommand(
            SubCommand::with_name("add").about("Adds a new task").arg(
                Arg::with_name("DESCRIPTION")
                    .help("The task description")
                    .required(true),
            ),
        )
        .subcommand(SubCommand::with_name("list").about("Lists all tasks"))
        .subcommand(
            SubCommand::with_name("remove")
                .about("Removes a task")
                .arg(Arg::with_name("ID").help("The task ID").required(true)),
        )
        .subcommand(
            SubCommand::with_name("complete")
                .about("Marks a task as completed")
                .arg(Arg::with_name("ID").help("The task ID").required(true)),
        );
}
