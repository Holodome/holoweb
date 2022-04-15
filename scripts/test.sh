#!/usr/bin/env bash

set -x
set -eo pipefail

TEST_THREADS=${TEST_THREADS:=8}

# This is needed because, at least on my macbook air 2020, file limit is set to 256 which is
# insufficient for running multiple tests simultaneously
ulimit -n 1024
cargo test -- --test-threads="$TEST_THREADS"
