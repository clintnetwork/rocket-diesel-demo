# Rocket Diesel Demo

This is a PoC app demonstrating how [Rocket](rocket.rs) and [Diesel](diesel.rs) work together.

## Prerequisites
Postgres must be running and have the `rocket-diesel-demo` db created

## Setup

```bash
cargo run -- --config habitat/default.toml
curl localhost:8000/hello -d '{"name":"bar"}' -H "Content-Type: application/json"
curl localhost:8000/world
```

## Development

This project uses Habitat for development. 

* Run `hab studio enter` to enter a development clean room.
* `build-api` - Build a new version of the api
* * It will automatically be reloaded for you

## Build with your own Habitat origin

* Run `HAB_STUDIO_NOSTUDIORC=1 hab studio enter`
* `build`
* `hab pkg upload $HAB_ORIGIN/rocket-diesel-demo`
