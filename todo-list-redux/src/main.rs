mod todo;
mod store;

use todo::TodoAction::{Add, Toggle, Remove};
use store::Action::{Todos, Visibility};
use store::VisibilityFilter::{ShowAll, ShowActive, ShowCompleted};
use todo::Todo;
use store::{Store, State, reducer};


fn print_todo(todo: &Todo) {
    let done = if todo.completed {
        "✔"
    } else {
        " "
    };
    println!("[{}] {} {}", done, todo.id, todo.title);
}

fn print_todos(state: &State) {
    let visibility = &state.visibility_filter;
    println!("\n\nTodo List:\n-------------------");
    for todo in &state.todos {
        if !todo.deleted {
            match *visibility {
                ShowAll => print_todo(&todo),
                ShowCompleted => {
                    if todo.completed {
                        print_todo(&todo)
                    }
                }
                ShowActive => {
                    if !todo.completed {
                        print_todo(&todo)
                    }
                }
            }
        }
    }
    println!("-------------------\nVisibility filter:  {:?}", visibility);
    print_instructions();
}

fn print_instructions() {
    println!("\nAvailable commands: \nadd [text] - toggle [id] - remove [id]\nshow \
              [all|active|completed]");
}

fn invalid_command(command: &str) {
    println!("Invalid command: {}", command);
}


fn main() {
    let mut store = Store::create_store(reducer);
    store.subscribe(print_todos);

    print_instructions();
    loop {
        let mut command = String::new();
        std::io::stdin()
            .read_line(&mut command)
            .expect("failed to read line");

        let command_parts: Vec<&str> = command.split_whitespace().collect();

        match command_parts.len() {
            0 => invalid_command(&command),
            _ => {
                match command_parts[0] {
                    "add" => store.dispatch(Todos(Add(command_parts[1..].join(" ").to_string()))),
                    "remove" => {
                        if let Ok(num) = command_parts[1].parse::<i16>() {
                            store.dispatch(Todos(Remove(num)))
                        }
                    }
                    "toggle" => {
                        if let Ok(num) = command_parts[1].parse::<i16>() {
                            store.dispatch(Todos(Toggle(num)))
                        }
                    }
                    "show" => {
                        match command_parts[1] {
                            "all" => store.dispatch(Visibility(ShowAll)),
                            "active" => store.dispatch(Visibility(ShowActive)),
                            "completed" => store.dispatch(Visibility(ShowCompleted)),
                            _ => invalid_command(&command),
                        }
                    }
                    _ => invalid_command(&command),
                }
            }
        }
    }
}
