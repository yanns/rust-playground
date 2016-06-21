use Action::{Todos, Visibility};
use TodoAction::{Add, Toggle, Remove};
use VisibilityFilter::{ShowAll, ShowActive, ShowCompleted};

#[derive(Clone, Debug)]
struct Todo {
    id: i16,
    title: String,
    completed: bool,
    deleted: bool,
}

impl Todo {
    pub fn new(id: i16, title: String) -> Todo {
        Todo {
            id: id,
            title: title,
            completed: false,
            deleted: false,
        }
    }
}

#[derive(Clone, Debug)]
struct State {
    todos: Vec<Todo>,
    visibility_filter: VisibilityFilter,
}

impl State {
    fn default() -> State {
        State {
            todos: Vec::new(),
            visibility_filter: VisibilityFilter::ShowAll,
        }
    }
}

#[derive(Clone, Debug)]
enum VisibilityFilter {
    ShowAll,
    ShowActive,
    ShowCompleted,
}

#[derive(Debug)]
enum Action {
    Todos(TodoAction),
    Visibility(VisibilityFilter),
}

#[derive(Debug)]
enum TodoAction {
    Add(String),
    Toggle(i16),
    Remove(i16),
}

fn reducer(state: &State, action: Action) -> State {
    // Always return a new State
    State {
        todos: todo_reducer(&state.todos, &action),
        visibility_filter: visibility_reducer(&state.visibility_filter, &action),
    }
}

fn get_mut_todo(todos: &mut Vec<Todo>, todo_id: i16) -> Option<&mut Todo> {
    todos.iter_mut().find(|todo| todo.id == todo_id)
}

fn todo_reducer(state: &Vec<Todo>, action: &Action) -> Vec<Todo> {
    let mut new_state: Vec<Todo> = state.clone();

    match *action {
        Todos(ref todo_action) => {
            match *todo_action {
                Add(ref title) => {
                    let new_id = new_state.len() as i16 + 1;
                    new_state.push(Todo::new(new_id, title.to_string()))
                }
                Toggle(todo_id) => {
                    if let Some(todo) = get_mut_todo(&mut new_state, todo_id) {
                        todo.completed = !todo.completed
                    }
                }
                Remove(todo_id) => {
                    if let Some(todo) = get_mut_todo(&mut new_state, todo_id) {
                        todo.deleted = true
                    }
                }
            }
        }
        _ => (),
    }

    new_state
}

fn visibility_reducer(state: &VisibilityFilter, action: &Action) -> VisibilityFilter {
    match *action {
        Visibility(ref vis_action) => vis_action.clone(),
        _ => state.clone(),
    }
}

struct Store {
    state: State,
    listeners: Vec<fn(&State)>,
    reducer: fn(&State, Action) -> State,
}

impl Store {
    fn create_store(reducer: fn(&State, Action) -> State) -> Store {
        Store {
            state: State::default(),
            listeners: Vec::new(),
            reducer: reducer,
        }
    }

    fn subscribe(&mut self, listener: fn(&State)) {
        &self.listeners.push(listener);
    }

    #[allow(dead_code)]
    fn get_state(&self) -> &State {
        &self.state
    }

    fn dispatch(&mut self, action: Action) {
        self.state = (self.reducer)(&self.state, action);
        for listener in &self.listeners {
            listener(&self.state)
        }
    }
}

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
