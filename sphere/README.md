Test project to use the [commercetools platform](http://dev.commercetools.com/)

## Development

Usage:
```
cargo run -- <PROJECT_KEY> <CLIENT_ID> <CLIENT_SECRET>
```

With log for `auth` module:
```
RUST_LOG=sphere::auth cargo run -- <PROJECT_KEY> <CLIENT_ID> <CLIENT_SECRET>
```

## Release

Create an executable:
```
cargo build
```

Run the executable
```
./target/debug/sphere <PROJECT_KEY> <CLIENT_ID> <CLIENT_SECRET>
```
