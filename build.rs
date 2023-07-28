use std::{
    collections::HashSet,
    path::{Path, PathBuf},
    process::Command,
    str::FromStr,
};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let r_args = ["--vanilla", "--silent", "--no-echo"];
    // requires that either R is available in path or that R_HOME is set.
    let r_available = Command::new("R")
        .args(r_args)
        .args(["-e", "cat(R.home())"])
        .output();
    // dbg!(&r_available);

    let r_home = match r_available {
        Ok(output) => String::from_utf8(output.stdout).unwrap(),
        Err(_error) => {
            // alright, but is R_HOME set?
            std::env::var("R_HOME").expect("Environment variable `R_HOME` is not set.")
        }
    };
    let r_home = PathBuf::from_str(&r_home).unwrap().canonicalize().unwrap();
    let r_include = r_home.join("include");

    let r_headers = read_dir_recursively(r_include.as_path());
    dbg!(&r_home);
    dbg!(&r_headers);
    // FIXME: 
    let r_path = r_home.join("bin/x64/R");
}

fn read_dir_recursively(root: &Path) -> Vec<PathBuf> {
    let mut result = Vec::new();
    // FIXME: maybe this let should be reversed?
    let mut candidates: Vec<_> = root.read_dir().unwrap().flatten().collect();
    // candidates.reverse();
    while let Some(candidate) = candidates.pop() {
        if candidate.path().is_dir() {
            candidates.extend(candidate.path().read_dir().unwrap().flatten());
        } else {
            result.push(candidate.path());
        }
    }
    result
}
