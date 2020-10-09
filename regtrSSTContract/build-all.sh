#!/usr/bin/env bash

# pushd accContract/roleContract && cargo +nightly contract build && popd &&
pushd roleContract && cargo +nightly contract build && popd &&
cargo +nightly contract build
cargo contract generate-metadata