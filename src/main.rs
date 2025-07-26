use std::env;
use std::process::Command;

fn main() {
    // Ensure the library is built
    use kill_the_creeps as _;

    let godot = env::var("GODOT4_BIN").expect("GODOT4_BIN environment variable not set");
    println!("Starting Godot from: {}", godot);

    let status = Command::new(godot)
        .arg("--path")
        .arg("godot")
        .status()
        .expect("failed to start Godot");

    if !status.success() {
        panic!("Godot exited with status: {:?}", status);
    }
}
