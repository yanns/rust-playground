Test project to use the [commercetools platform](http://dev.commercetools.com/)

## Development

Usage:
```
cargo run --example fetch_products -- <PROJECT_KEY> <CLIENT_ID> <CLIENT_SECRET>
```

To output some logs, [configure the `RUST_LOG` environment variable](http://doc.rust-lang.org/log/env_logger/index.html)

Example: to enable all logs for the `auth` module:
```
RUST_LOG=sphere::auth cargo run --example fetch_products -- <PROJECT_KEY> <CLIENT_ID> <CLIENT_SECRET>
```

## Release an example

Create an executable from an example:
```
cargo build --example fetch_products
```

Run the executable
```
./target/debug/examples/fetch_products <PROJECT_KEY> <CLIENT_ID> <CLIENT_SECRET>
```
