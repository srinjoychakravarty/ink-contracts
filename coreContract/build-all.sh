#!/usr/bin/env bash

pushd roleContract && cargo +nightly contract build && popd &&
pushd regtrSSTContract && cargo +nightly contract build && popd &&
cargo +nightly contract build
cargo contract generate-metadata