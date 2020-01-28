#!/usr/bin/bash

#################
# Copies over files to make the wasm build (wasm-unknown-unknown) usable,
# then opens up a simple http server on port 8888 of local host for testing.
################

cp target/wasm32-unknown-unknown/release/cza.wasm html/
mv html/cza.wasm html/gate_app.wasm
cp howler.js html/
cd html
python -m SimpleHTTPServer 8888
