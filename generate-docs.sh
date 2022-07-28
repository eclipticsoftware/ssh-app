#!/bin/bash

cd ssh-tunnel
cargo doc --document-private-items --no-deps --target-dir ../doc  --features doc-images
cd ..

cd src-tauri
cargo doc --document-private-items --no-deps --target-dir ../doc
