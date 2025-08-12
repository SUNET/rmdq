# rmdq

A MDQ JSON server implementation in Rust.

## How to build?

First install Rust following https://rustup.rs and then follow along.

```
git clone https://github.com/SUNET/rmdq
cd rmdq
cargo build --release
./target/release/rmdq
```

## To load data

Use `./fetch_metadata.py` to create `webdata.json` in the same folder.
Then start the server `cargo run`


```
./fetch_metadata.py
cargo run
```

## Routes available

- http://localhost:8080/entities?q=swe
- http://localhost:8080/entities/%7Bsha1%7D5dc5772ac948105273633713fcc018fed26b1c3b.json


## Test data to try out


```
./fetch_metadata.py https://ta.oidfed.data.kit.edu/discover?trust_anchor=https://edugain.oidf.lab.surf.nl&entity_type=openid_provider
cargo run
```

To test a single query:

```
curl http://localhost:8080/entities?q=ris
```
