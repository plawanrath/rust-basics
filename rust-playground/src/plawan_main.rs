// @plawanrath

// Rust can discover modules but Rust doesn't auto-discover subfolders in a flat layout. So you tell it where the files are
#[path = "plawan/starter.rs"]
mod starter;

#[path = "plawan/starter_test.rs"]
#[cfg(test)]
mod starter_test;

fn main() {
    starter::run();
    // add your modules below
}