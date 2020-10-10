#!/usr/bin/env bash

pushd roleContract && cargo +nightly-2020-07-12 contract build && popd &&
cargo +nightly-2020-07-12 contract build
cargo +nightly-2020-07-12 contract generate-metadata

