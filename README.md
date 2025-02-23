# rmdq

A MDQ JSON server implementation in Rust.

## How to build?

First install Rust following https://rustup.rs and then follow along.

```
git clone https://github.com/kushaldas/rmdq
cd rmdq
cargo build --release
./target/release/rmdq
```

## To load data

The server starts with an empty state (we can change it later).
Get the latest `ds.json` and use `./parse_json.py` to create `webdata.json` in the same folder.

Now you can call `http://localhost:8080/update` to load the latest version of the data on the app (from the json file).

```
curl -O https://ds.environment.tld/ds.json
./parse_json.py
curl http://localhost:8080/update
```

## Routes available

- http://localhost:8080/entities?q=sol
- http://localhost:8080/entities/%7Bsha1%7D5dc5772ac948105273633713fcc018fed26b1c3b.json


## Test data to try out

In a different terminal start the application first.

```
curl https://raw.githubusercontent.com/TheIdentitySelector/thiss-mdq/refs/heads/master/test/edugain.json -o sp.json
python3 parse_json.py
curl http://localhost:8080/update  # to load the data
```

To test a single query:

```
curl http://localhost:8080/entities?q=sab
```
