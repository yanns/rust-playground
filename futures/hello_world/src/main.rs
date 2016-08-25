extern crate futures;
extern crate futures_io;
extern crate futures_mio;
extern crate futures_tls;

use std::net::ToSocketAddrs;

use futures::Future;
use futures_mio::Loop;
use futures_tls::ClientContext;

fn main() {
    let mut lp = Loop::new().unwrap();
    let addr = "www.rust-lang.org:443".to_socket_addrs().unwrap().next().unwrap();

    let socket = lp.handle().tcp_connect(&addr);

    let tls_handshake = socket.and_then(|socket| {
        let cx = ClientContext::new().unwrap();
        cx.handshake("www.rust-lang.org", socket)
    });
    let request = tls_handshake.and_then(|socket| {
        futures_io::write_all(socket, "\
    		GET / HTTP/1.0\r\n\
            Host: www.rust-lang.org\r\n\
            \r\n\
        ".as_bytes())
    });
    let response = request.and_then(|(socket, _)| futures_io::read_to_end(socket, Vec::new()));

    let (_, data) = lp.run(response).unwrap();
    println!("{}", String::from_utf8_lossy(&data));
}
