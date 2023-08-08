use std::process::Command;

pub fn main() {
    // Command::new("npx")
    //     .args(&[
    //         "tailwindcss",
    //         "-i",
    //         "./web/input.css",
    //         "-o",
    //         "./web/main.css",
    //     ])
    //     .status()
    //     .unwrap();
    println!("cargo:rerun-if-changed=src/*.rs");
}
