#[cfg(feature = "generate_bindings")]
use rayon::prelude::*;
#[cfg(feature = "generate_bindings")]
use std::{path::PathBuf, process::Command, str::FromStr};

// TODO: don't bother with Rdefines.h

// FIXME: Why is Rboolean included everywhere?

fn main() {
    #[cfg(feature = "generate_bindings")]
    generate_bindings();
}

#[cfg(feature = "generate_bindings")]
fn generate_bindings() {
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

    // FIXME: other platforms
    let r_path = r_home.join("bin/x64/R");
    let crate_root: PathBuf = env!("CARGO_MANIFEST_DIR").into();

    let r_headers = read_dir_recursively(r_include.as_path());
    let r_file_item_list: Vec<_> = r_headers
        .iter()
        .map(|x| x.strip_prefix(&r_include).unwrap().to_str().unwrap())
        .map(|r_header| r_header.replace(r"\", r"/").replace(".", r"\."))
        .map(|x| format!(".*{}", x))
        .collect();

    // rayon::ThreadPoolBuilder::new()
    //     .num_threads(1)
    //     .build_global()
    //     .unwrap();
    r_headers
        .into_par_iter()
        .zip(r_file_item_list.clone().into_par_iter())
        // .par_iter()
        .for_each(|(full_r_header, item_list_r_header)| {
            let mut binder = bindgen::builder()
                .layout_tests(false)
                .sort_semantically(true)
                // does nothing
                // .translate_enum_integer_types(true)
                // .clang_arg("-std=c11")
                // .clang_arg("-std=c17")
                .clang_arg("-std=c2x")
                // .header("wrapper_head.h")
                // .blocklist_file(".*stddef\\.h")
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
                // .allowlist_recursively(false)
                .clang_arg(format!("-I{}", (&r_include).display()));

            // output path and name + extension
            let bind_header = full_r_header
                .strip_prefix(&r_include)
                .unwrap()
                .with_extension("rs");
            let bind_header = crate_root.join("src").join("bindings").join(bind_header);

            std::fs::create_dir_all(bind_header.parent().unwrap()).unwrap();

            let specific_header = full_r_header
                .strip_prefix(&r_include)
                .unwrap()
                .to_str()
                .unwrap();
            // dbg!(&specific_header);

            println!("specific header: {}", specific_header);
            println!("item_list_r_header: {}", item_list_r_header);
            // block all the other r-headers
            for r_header in &r_file_item_list {
                if r_header == &item_list_r_header {
                    // don't block current header being processed
                    continue;
                }
                println!("blocking_the_rest: {}", &r_header);
                binder = binder.blocklist_file(r_header);
            }

            match specific_header {
                "Rmath.h" => {
                    // FIXME: so this causes a repetition that I'm not sure I particularly like.
                    binder = binder.header("wrapper_head_Rcomplex.h");
                }
                // TODO: Add `R_ext\\Complex.h` and use the `Rcomplex` we defined in `wrapper_head_Rcomplex.h`
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
                    return;
                }
                _ => {}
            }

            binder
                .header(full_r_header.to_str().unwrap())
                .generate()
                .unwrap()
                .write_to_file(bind_header)
                .unwrap();
        });
}

#[cfg(feature = "generate_bindings")]
/// Returns all the paths to the files under `root` recursively.
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
