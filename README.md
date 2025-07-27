# Kill the Creeps

This repository contains the **Kill the Creeps** example for the
[godot-rust](https://github.com/godot-rust/gdext) bindings. The Godot
project files live in the `godot/` directory and the Rust source for the
GDExtension library lives in `src/`.

## Running

Make sure the `GODOT4_BIN` environment variable points to your Godot 4
executable. Then simply run:

```bash
cargo run
```

This builds the Rust library and launches Godot with the demo project.
