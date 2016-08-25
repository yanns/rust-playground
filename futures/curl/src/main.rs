extern crate curl;
extern crate futures;
extern crate futures_curl;
extern crate futures_io;
extern crate futures_mio;

use std::io::Write;
use std::sync::Arc;
use std::sync::Mutex;
use std::cell::RefCell;

use curl::easy::Easy;
use futures::{BoxFuture, Future};
use futures_io::IoFuture;
use futures_mio::{Loop, LoopPin};
use futures_curl::Session;

fn get_page(lp: LoopPin, sess: Session) -> BoxFuture<String, std::io::Error> {
	let response = Arc::new(lp.add_loop_data(RefCell::new(Vec::new())));
	let response1 = response.clone();

	let mut req = Easy::new();
	req.get(true).unwrap();
	req.url("https://www.rust-lang.org").unwrap();

	req.write_function(move |data| {
		response1.get().unwrap().borrow_mut().extend_from_slice(data);
		Ok(data.len())
	}).unwrap();

	sess.perform(req)
		.map(move |_| {
			let s = response.get().unwrap().borrow();
			String::from_utf8_lossy(&s).into_owned()
		})
		.boxed()
}

fn main() {
	let mut lp = Loop::new().unwrap();
	let sess = Session::new(lp.pin());

	let f = get_page(lp.pin(), sess);
	println!("{}", lp.run(f).unwrap());
}
