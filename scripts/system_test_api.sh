#!/usr/bin/env bash
set -x
set -eo pipefail

pushd tests/system/api

pytest

popd
