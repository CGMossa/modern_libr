use std::{path::PathBuf, process::Command, str::FromStr};

// TODO: don't bother with Rdefines.h

// FIXME: Why is Rboolean included everywhere?

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
    // let r_home = PathBuf::from_str(&r_home).unwrap().canonicalize().unwrap();
    let r_home = PathBuf::from_str(&r_home).unwrap();
    let r_include = r_home.join("include");

    let r_headers = read_dir_recursively(r_include.as_path());
    // dbg!(&r_home);
    // dbg!(&r_headers);
    // FIXME: other platforms
    let r_path = r_home.join("bin/x64/R");

    let binder = bindgen::builder()
        .layout_tests(false)
        .header("wrapper_head.h")
        .blocklist_file(".*stddef\\.h")
        .blocklist_file(".*wrapper_header\\.h")
        .merge_extern_blocks(true)
        // does nothing
        // .generate_block(true)
        // does nothing
        // .generate_comments(true)
        // does nothing?
        .clang_arg("-fparse-all-comments")
        // does something
        // .generate_cstr(true)
        // does nothing
        // .size_t_is_usize(true)
        .rustified_enum(".*")
        //FIXME: enable this maybe?
        .allowlist_recursively(false)
        .clang_arg(format!("-I{}", r_include.display()));

    let crate_root: PathBuf = env!("CARGO_MANIFEST_DIR").into();

    for header in &r_headers {
        let bind_header = header
            .strip_prefix(&r_include)
            .unwrap()
            .with_extension("rs");
        let bind_header = crate_root.join("src").join("bindings").join(bind_header);

        std::fs::create_dir_all(bind_header.parent().unwrap()).unwrap();
        // dbg!(&bind_header);

        let mut binder = binder.clone();

        let specific_header = header.strip_prefix(&r_include).unwrap().to_str().unwrap();
        match specific_header {
            "Rmath.h" => {
                binder = binder.header("wrapper_head_Rcomplex.h");
            }
            "R_ext\\Parse.h" => {
                binder = binder.header("Rinternals.h");
                binder = binder.blocklist_file(".*Rinternals\\.h");
            }
            "R_ext\\Altrep.h" => {
                binder = binder.header("Rinternals.h");
                binder = binder.blocklist_file(".*Rinternals\\.h");
            }
            "R_ext\\GraphicsEngine.h" | "R_ext\\GraphicsDevice.h" | "R_ext\\Connections.h" => {
                // ignore for now
                continue;
            }
            _ => {}
        }

        binder
            .header(header.to_str().unwrap())
            .generate()
            .unwrap()
            .write_to_file(bind_header)
            .unwrap();
    }
}

fn read_dir_recursively(root: &std::path::Path) -> Vec<PathBuf> {
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
