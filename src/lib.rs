use std::env;
use std::fs;
use std::path::{PathBuf};

pub fn load_input(bin_name: &str) -> String {
    let base_dir = env!("CARGO_MANIFEST_DIR");

    let input_path = PathBuf::from(base_dir).join("inputs").join(bin_name);

    println!("Reading input from file {}", input_path.as_path().display());
    let contents =
        fs::read_to_string(input_path).expect("failed to read file");
    contents
}
