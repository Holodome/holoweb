# rustweb

This is a blog/forum/personal website project. The goal is to create MVP webserver
implementing backend infrastructure.

## Developer Quick Start

You should have docker installed to run Redis.
In order to view logs in human-readable format you can use [bunyan](https://crates.io/crates/bunyan),
but it is not necessary to run application.

To run locally execute commands in following order:
```shell
git clone https://github.com/Holodome/rustweb.git
cd rustweb
srcipts/init_redis.sh
cargo run
```

The server should start now.

## Disclaimer

This is my first nontrivial backend project, as well as first project in Rust.
One of the reasons it was created to get more fluent in language, as well as
more comfortable with server backend.
