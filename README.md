# Rocket Diesel Demo

This is a PoC app demonstrating how [Rocket](rocket.rs) and [Diesel](diesel.rs) work together.

## Prerequisites
Postgres must be running and have the `diesel-rocket-demo` db created

## Setup

```bash
cargo run -- --config config.sample.toml
curl localhost:8000/hello -d '{"name":"bar"}' -H "Content-Type: application/json"
curl localhost:8000/world
```