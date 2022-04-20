# holoweb

[![Integration testing](https://github.com/Holodome/holoweb/actions/workflows/integration-testing.yml/badge.svg?branch=master)](https://github.com/Holodome/holoweb/actions/workflows/integration-testing.yml)
[![System testing](https://github.com/Holodome/holoweb/actions/workflows/system-testing.yml/badge.svg?branch=master)](https://github.com/Holodome/holoweb/actions/workflows/system-testing.yml)
[![Deploy](https://github.com/Holodome/holoweb/actions/workflows/deploy.yml/badge.svg?branch=master)](https://github.com/Holodome/holoweb/actions/workflows/deploy.yml)

This is a blog/forum/personal website project. The goal is to create MVP webserver
implementing backend infrastructure.

## Goal

The goal is to create function personal website application trying to follow best practices.
The project aims to be as simple as possible, while maintaining acceptable functionality.

## Tech stack

* Framework: [actix-web](https://actix.rs)
* Database: [sqlite](https://www.sqlite.org/index.html)
* ORM: [diesel](https://diesel.rs/)
* In-memory database: [redis](https://redis.io/)
* CSS: [semantic ui](https://semantic-ui.com/)

For details about rust implementation see Cargo.toml.

Docker is used to containerize application.

Github actions are used for CI/CD.

## Developer Quick Start

You should have docker installed to run Redis.
In order to view logs in human-readable format you can use [bunyan](https://crates.io/crates/bunyan),
though it is not necessary to run application.
You also need [diesel](https://diesel.rs/) cli to run migrations on database.

To run locally execute commands in following order:

```shell
scripts/init_redis.sh
scripts/init_db.sh
cargo run # | bunyan
```

The server should start now.

## Testing

There are several layers of testing. Unit testing and integration testing is handled
by rust, and system testing is done using python.

To run tests locally execute commands in following order:

```shell
scripts/init_redis.sh
scripts/init_db.sh
cargo test
```

### Details

Unit testing is not numerous due to mostly type-based API used internally.
It generally covers modules related to parsing data, as main application code
cannot be tested individually.

Integration testing covers testing of database and API. It generally creates new instance
of database/server in each test.

System testing covers work of application as a whole. It applies to testing ui using
[chromedriver](https://chromedriver.chromium.org/), and testing application API again.

## Project structure

* tests/ - Directory with all tests, written in both rust and other languages.
* templates/ - internal data used by server. These are the html templates that are later embedded in application.
* static/ - static website content.
* src/ - source code.
* scripts/ - helper scripts used in CI/CD or locally.
* migrations/ - database migrations generated using diesel, must be run to initialize database.
* config/ - server startup configuration.

Additional directories:
* internal/ - data related to development. This may not persist.
* .data/ - volume directory generate by docker-compose. Appears only on local runs of production builds.
* .github/ - github actions integration

## Disclaimer

This is my first nontrivial backend project, as well as first project in Rust.
One of the reasons it was created is to get more fluent in the language.
