#!/usr/bin/env bash

set -o xtrace
set -o errexit

cargo bin wasm-pack build --target nodejs 
node index.js