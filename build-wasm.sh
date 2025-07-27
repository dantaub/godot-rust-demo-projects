#!/bin/sh
# Copyright (c) godot-rust; Bromeon and contributors.
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

# Must be in the project root in order to pick up the .cargo/config
cd "$(dirname "$0")"

# Build the host gdextension first so that the Godot editor doesn't complain.
cargo +nightly build --package kill-the-creeps &&
cargo +nightly build --package kill-the-creeps \
  --features godot/experimental-wasm,godot/lazy-function-tables \
  --target wasm32-unknown-emscripten -Zbuild-std $@
