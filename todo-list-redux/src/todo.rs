use store::Action;
use store::Action::Todos;
use todo::TodoAction::{Add, Toggle, Remove};

#[derive(Clone, Debug, RustcEncodable, RustcDecodable)]
pub struct Todo {
    pub id: i16,
    pub title: String,
    pub completed: bool,
    pub deleted: bool,
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

#[derive(Debug)]
pub enum TodoAction {
    Add(String),
    Toggle(i16),
    Remove(i16),
}

fn get_mut_todo(todos: &mut Vec<Todo>, todo_id: i16) -> Option<&mut Todo> {
    todos.iter_mut().find(|todo| todo.id == todo_id)
}

pub fn todo_reducer(state: &Vec<Todo>, action: &Action) -> Vec<Todo> {
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
