// Rust can discover modules but Rust doesn't auto-discover subfolders in a flat layout. So you tell it where the files are
#[path = "pushpita/starter.rs"]
mod starter;

fn main() {
    starter::run();
    // add your modules below
}