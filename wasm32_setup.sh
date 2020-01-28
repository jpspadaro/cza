#!/usr/bin/bash

#################
# Copies over files to make the wasm build (wasm-unknown-unknown) usable,
# then opens up a simple http server on port 8888 of local host for testing.
################

cp target/wasm32-unknown-unknown/release/chirperjax.wasm html/
mv html/chirperjax.wasm html/gate_app.wasm
cp howler.js html/
cd html
python -m SimpleHTTPServer 8888
