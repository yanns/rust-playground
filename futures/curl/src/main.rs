extern crate curl;
extern crate futures;
extern crate futures_curl;
extern crate futures_io;
extern crate futures_mio;
extern crate time;

use std::sync::Arc;
use std::cell::RefCell;
use std::thread;
use std::sync::mpsc;

use curl::easy::Easy;
use futures::{BoxFuture, Future};
use futures_mio::{Loop, LoopHandle};
use futures_curl::Session;

struct Api {
    lp: LoopHandle,
    sess: Session,
}

impl Api {
    fn new(lp: LoopHandle, sess: Session) -> Api {
        Api {
            lp: lp,
            sess: sess,
        }
    }

    fn get_page(&self) -> BoxFuture<String, std::io::Error> {

        let sess = self.sess.clone();

        // First one for response and second for the headers
        self.lp
            .add_loop_data(|| (RefCell::new(Vec::<u8>::new()), RefCell::new(Vec::<Vec<u8>>::new())))
            .and_then(move |data| {
                let data = Arc::new(data);

                let data1 = data.clone();
                let data2 = data.clone();

                let mut req = Easy::new();
                req.get(true).unwrap();
                req.url("https://www.rust-lang.org").unwrap();

                req.write_function(move |d| {
                        data1.get().unwrap().0.borrow_mut().extend_from_slice(d);
                        Ok(d.len())
                    })
                    .unwrap();

                req.header_function(move |header| {
                        data2.get().unwrap().1.borrow_mut().push(header.to_vec());
                        true
                    })
                    .unwrap();

                sess.perform(req)
                    .map(move |(mut resp, err)| {
                        assert!(err.is_none());
                        assert_eq!(resp.response_code().unwrap(), 200);

                        let response = data.get().unwrap().0.borrow();
                        let response = String::from_utf8_lossy(&response);
                        assert!(response.contains("<html>"));
                        let headers = data.get()
                            .unwrap()
                            .1
                            .borrow()
                            .iter()
                            .map(|h| String::from(String::from_utf8_lossy(&h).trim()))
                            .fold(String::new(), |acc, elem| acc + &elem + "\n");
                        assert!(headers.len() > 0);

                        format!("Headers:\n\n{}\n\nResponse:\n\n{}", headers, response)
                    })
            })
            .boxed()
    }
}

fn main() {
    let (tx, rx) = mpsc::channel();

    // Make sure the event loop runs forever in a new thread.
    thread::spawn(move || {
        // Create an event loop that we'll run on, as well as an HTTP `Session`
        // which we'll be routing all requests through.
        let mut lp = Loop::new().unwrap();

        // Send loop handler and Session to the main thread so that async curl requests can be
        // made.
        tx.send((lp.handle(), Session::new(lp.pin()))).unwrap();

        lp.run(futures::empty::<(), ()>()) // Keep the event loop running forever
            .unwrap();
    });

    let (lph, sess) = rx.recv().unwrap();

    // Create a new API using the loop handle and Session.
    let api = Api::new(lph, sess);

    // Wait for the page and quit.
    let res = await(api.get_page());
    println!("{}", res.unwrap());
}

// Taken from tokio and modified to compile with upstream changes.
// We don't use Future.wait() because it pins the Future to the current thread and that messes up
// LoopData.get().
fn await<T: Future + Send + 'static>(f: T) -> Result<T::Item, T::Error>
    where T::Item: Send,
          T::Error: Send
{
    let (tx, rx) = mpsc::channel();

    f.then(move |res| {
            tx.send(res).unwrap();
            Ok::<(), ()>(())
        })
        .forget();

    rx.recv().unwrap()
}