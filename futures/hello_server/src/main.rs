extern crate futures;
extern crate futures_io;
extern crate futures_mio;

use futures::Future;
use futures::stream::Stream;
use futures_mio::Loop;

fn main() {
    let mut lp = Loop::new().unwrap();
    let address = "127.0.0.1:8080".parse().unwrap();
    let listener = lp.handle().tcp_listen(&address);

    let server = listener.and_then(|listener| {
        let addr = listener.local_addr().unwrap();
        println!("Listening for connections on {}", addr);

        let clients = listener.incoming();
        let welcomes = clients.map(|(socket, _peer_addr)| {
            futures_io::write_all(socket, b"Hello!\n")
        });
        welcomes.for_each(|future| {
        	future.forget();
            Ok(())
        })
    });

    lp.run(server).unwrap();
}