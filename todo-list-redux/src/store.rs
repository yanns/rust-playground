use store::Action::Visibility;
use todo::{Todo, TodoAction, todo_reducer};

#[derive(Clone, Debug)]
pub struct State {
    pub todos: Vec<Todo>,
    pub visibility_filter: VisibilityFilter,
}

impl State {
    pub fn default() -> State {
        State {
            todos: Vec::new(),
            visibility_filter: VisibilityFilter::ShowAll,
        }
    }
}

#[derive(Debug)]
pub enum Action {
    Todos(TodoAction),
    Visibility(VisibilityFilter),
}

#[derive(Clone, Debug)]
pub enum VisibilityFilter {
    ShowAll,
    ShowActive,
    ShowCompleted,
}

pub fn reducer(state: &State, action: Action) -> State {
    // Always return a new State
    State {
        todos: todo_reducer(&state.todos, &action),
        visibility_filter: visibility_reducer(&state.visibility_filter, &action),
    }
}

fn visibility_reducer(state: &VisibilityFilter, action: &Action) -> VisibilityFilter {
    match *action {
        Visibility(ref vis_action) => vis_action.clone(),
        _ => state.clone(),
    }
}

pub struct Store {
    state: State,
    listeners: Vec<fn(&State)>,
    reducer: fn(&State, Action) -> State,
}

impl Store {
    pub fn create_store(reducer: fn(&State, Action) -> State) -> Store {
        Store {
            state: State::default(),
            listeners: Vec::new(),
            reducer: reducer,
        }
    }

    pub fn subscribe(&mut self, listener: fn(&State)) {
        &self.listeners.push(listener);
    }

    #[allow(dead_code)]
    pub fn get_state(&self) -> &State {
        &self.state
    }

    pub fn dispatch(&mut self, action: Action) {
        self.state = (self.reducer)(&self.state, action);
        for listener in &self.listeners {
            listener(&self.state)
        }
    }
}
