// This macro basically takes in a module name as a parameter, and auto brings them all into scope as a public module
macro_rules! mod_days {
    ($($x:ident),*) => {
        $(
            pub mod $x;
        )*
    };
}

mod_days!(one, one_two, two, two_two, three);

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
