#[macro_use]
extern crate nickel;
extern crate rustc_serialize;
extern crate handlebars;

use nickel::{Nickel, HttpRouter, FormBody};
use std::sync::{Arc, Mutex};

mod todo;
mod store;
mod template;

use todo::TodoAction::{Add, Toggle, Remove};
use store::Action::{Todos, Visibility};
use store::VisibilityFilter::{ShowAll, ShowActive, ShowCompleted};
use store::{Store, reducer};
use template::render;


fn main() {
    let mut store = Store::create_store(reducer);

    // Add some todos so we've got something to render
    store.dispatch(Todos(Add("one thing".to_string())));
    store.dispatch(Todos(Add("another thing".to_string())));

    let store_container = Arc::new(Mutex::new(store));

    let mut server = Nickel::new();

    let store = store_container.clone();
    server.get("/",
               middleware! { |_req, res|
        let store = store.lock().unwrap();

        return render(res, "./src/todos.tpl", store.get_state())
    });

    let store = store_container.clone();
    server.get("/:action/:id",
               middleware! { |_req, res|
        let mut store = store.lock().unwrap();

        if let Ok(num) = _req.param("id").unwrap().parse::<i16>() {
            match _req.param("action").unwrap() {
                "toggle" => store.dispatch( Todos( Toggle(num) ) ),
                "remove" => store.dispatch( Todos( Remove(num) ) ),
                _ => (),
            }
        } else {
            match _req.param("action").unwrap() {
                "show" => {
                    match _req.param("id").unwrap() {
                        "all" => store.dispatch( Visibility( ShowAll ) ),
                        "active" => store.dispatch( Visibility( ShowActive ) ),
                        "completed" => store.dispatch( Visibility( ShowCompleted ) ),
                        _ => (),
                    }
                },
                _ => (),
            }
        }

        return render(res, "./src/todos.tpl", store.get_state())

    });

    let store = store_container.clone();
    server.post("/*",
                middleware! { |req, res|
        let mut store = store.lock().unwrap();
        let form_body = req.form_body().ok().unwrap();
        if let Some(new_todo) = form_body.get("todo") {
            if new_todo.len() > 0 {
                store.dispatch( Todos( Add(new_todo.to_string()) ) );
            }
        }

        return render(res, "./src/todos.tpl", store.get_state())
    });

    server.listen("localhost:3000");
}
