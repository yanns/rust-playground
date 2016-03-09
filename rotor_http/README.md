simple http server using https://github.com/tailhook/rotor-http.

needs the source of https://github.com/tailhook/rotor-http in `../../rotor-http`.

To run the threaded server:
```
THREADS=8 cargo run --bin threaded
```

examples:
- http://127.0.0.1:3000
- http://127.0.0.1:3000/num
- http://127.0.0.1:3000/yann

