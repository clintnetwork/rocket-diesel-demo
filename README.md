# Rocket Diesel Demo

This is a PoC app demonstrating how [Rocket](rocket.rs) and [Diesel](diesel.rs) work together.

## Prerequisites
Postgres must be running

## Setup

```bash
diesel migration run
cargo run
curl localhost:8000/hello -d '{"name":"bar"}' -H "Content-Type: application/json"
curl localhost:8000/world
```

## Nifty commands

```bash
diesel migration pending
diesel migration list
diesel print-schema
```
