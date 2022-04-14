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
git clone https://github.com/Holodome/rustweb.git
cd rustweb
scripts/init_redis.sh
diesel migration run
cargo run  # | bunyan
```

The server should start now.

## Disclaimer

This is my first nontrivial backend project, as well as first project in Rust.
One of the reasons it was created is to get more fluent in the language.
