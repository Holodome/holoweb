# holoweb

This is a blog/forum/personal website project. The goal is to create MVP webserver
implementing backend infrastructure.

## Developer Quick Start

You should have docker installed to run Redis.
In order to view logs in human-readable format you can use [bunyan](https://crates.io/crates/bunyan),
though it is not necessary to run application.
You also need [diesel](https://diesel.rs/) cli to run migrations on database.

To run locally execute commands in following order:
```shell
git clone https://github.com/Holodome/holoweb.git
cd rustweb
scripts/init_redis.sh
scripts/init_db.sh
cargo run # | bunyan
```

The server should start now.

## Testing

There are several layers of testing. Unit testing and integration testing is handled
by rust, and system testing is done with python.

To run tests locally, you must initialize redis and database using scripts first.
Then execute *scripts/test.sh*.

## Project structure

* tests/ - Directory with all tests, written in both rust and other languages
* templates/ - internal data used by server. These are the html templates that are later embedded in application
* static/ - static website content
* src/ - source code
* scripts/ - helper scripts used in CI/CD or locally
* migrations/ - database migrations generated using diesel, must be run to initialize database
* config/ - server startup configuration

## Disclaimer

This is my first nontrivial backend project, as well as first project in Rust.
One of the reasons it was created is to get more fluent in the language.
