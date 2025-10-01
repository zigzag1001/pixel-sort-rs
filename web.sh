#!/bin/bash

wasm-pack build -t web --release -d web/pkg
cp web/* ~/storage/web/pixel-sort-rs -r
