pub mod days;
pub mod fun;

use std::env;
use std::fs;
use std::path::PathBuf;

pub struct State {
    pub dir: PathBuf,
}

impl State {
    pub fn input(&self, path: &str) -> String {
        let mut build_path = self.dir.clone();
        build_path.push(path);
        fs::read_to_string(build_path).unwrap()
    }
}

pub fn config() -> State {
    let dir: PathBuf = [env!("CARGO_MANIFEST_DIR"), "res"].iter().collect();

    State { dir }
}
