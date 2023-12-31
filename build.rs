#[cfg(feature = "generate_bindings")]
use std::{path::PathBuf, process::Command, str::FromStr};

// TODO: don't bother with Rdefines.h
// TODO: itermacros.h needs custom import.
// TODO: RS.h uses STRICT_R_HEADERS.
// TODO: use `get_r_library`

fn main() {
    // FIXME: use or remove these
    // let target = env::var("TARGET").expect("Could not get the target triple");
    // let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    // let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();

    #[cfg(feature = "generate_bindings")]
    generate_bindings();
}


/// Returns the path to the R library.
#[cfg(feature = "generate_bindings")]
fn get_r_library(r_home: &std::path::Path) -> PathBuf {
    use std::path::Path;
    let pkg_target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    match (cfg!(windows), pkg_target_arch.as_str()) {
        // For Windows
        (true, "x86_64") => Path::new(r_home).join("bin").join("x64"),
        (true, "x86") => Path::new(r_home).join("bin").join("i386"),
        (true, _) => panic!("Unknown architecture"),
        // For Unix-alike
        (false, _) => Path::new(r_home).join("lib"),
    }
}

#[cfg(feature = "generate_bindings")]
/// Generates and stores bindings for each r-header.
fn generate_bindings() {
    println!("cargo:rerun-if-changed=build.rs");

    let r_args = ["--vanilla", "--silent", "--no-echo"];
    // requires that either R is available in path or that R_HOME is set.
    let r_available = Command::new("R")
        .args(r_args)
        .args(["-e", "cat(normalizePath(R.home()))"])
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
    // FIXME: remove non `.h` or `.c` files from this
    // let r_headers = r_headers.iter().filter(|x|x.)
    let r_file_item_list: Vec<_> = r_headers
        .iter()
        .map(|x| x.strip_prefix(&r_include).unwrap().to_str().unwrap())
        .map(|r_header| r_header.replace(r"\", r"/").replace(".", r"\."))
        .map(|x| format!(".*{}", x))
        .collect();

    let r_file_item_list = &r_file_item_list;
    let crate_root = &crate_root;
    let r_include = &r_include;
    std::thread::scope(|s| {
        r_headers
            .into_iter()
            // .into_par_iter()
            .zip(
                r_file_item_list.clone().into_iter(), // .into_par_iter()
            )
            // .par_iter()
            .for_each(|(full_r_header, item_list_r_header)| {
                // let full_r_header = &full_r_header;
                // let item_list_r_header = &item_list_r_header;
                s.spawn(move || {
                    let mut binder = bindgen::builder()
                        .layout_tests(false)
                        .sort_semantically(true)
                        // does nothing
                        // .translate_enum_integer_types(true)
                        // .clang_arg("-std=c11")
                        // .clang_arg("-std=c17")
                        .clang_arg("-std=c2x")
                        .header("wrapper_head.h")
                        .blocklist_file(".*wrapper_head\\.h")
                        .merge_extern_blocks(true)
                        // does nothing
                        // .generate_block(true)
                        // does nothing
                        .generate_comments(true)
                        .clang_arg("-fparse-all-comments")
                        // does nothing?
                        // does something
                        // .generate_cstr(true)
                        // does nothing
                        // .size_t_is_usize(true)
                        
                        // `VECTOR_PTR` is deprecated, use `DATAPTR` and friends instead
                        .blocklist_item("VECTOR_PTR")
                        .wrap_unsafe_ops(true)
                        .rustified_enum(".*")
                        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
                        .parse_callbacks(Box::new(FixDocs))
                        .clang_arg(format!("-I{}", (&r_include).display()));

                    //TODO: add platform specific define, #define Win32 for example
                    // it is in `wrapper_head` right now,

                    if cfg!(windows) {
                        binder = binder.clang_arg("-DWin32");
                        binder = binder.clang_arg("-D_Win32");
                    }

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

                    // block all the other r-headers
                    for r_header in r_file_item_list {
                        if r_header == &item_list_r_header {
                            // don't block current header being processed
                            continue;
                        }
                        // println!("blocking_the_rest: {}", &r_header);
                        binder = binder.blocklist_file(r_header);
                    }

                    match specific_header {
                        r"R_ext/Complex.h" => {
                            binder = binder.header("wrapper_head_Rcomplex.h");
                        }
                        "R_ext\\Parse.h" => {
                            binder = binder.header("Rinternals.h");
                        }
                        "R_ext\\Altrep.h" => {
                            binder = binder.header("Rinternals.h");
                        }

                        "R_ext\\GraphicsEngine.h" => {
                            binder = binder.header("Rinternals.h");
                        }

                        "R_ext\\GraphicsDevice.h" => {
                            binder = binder.header("Rinternals.h");
                            binder = binder.header("R_ext\\GraphicsEngine.h");
                        }
                        "R_ext\\Connections.h" => {
                            binder = binder.header("Rinternals.h");
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
            });
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

#[cfg(feature = "generate_bindings")]
#[derive(Debug)]
struct FixDocs;

#[cfg(feature = "generate_bindings")]
impl bindgen::callbacks::ParseCallbacks for FixDocs {
    fn will_parse_macro(&self, _name: &str) -> bindgen::callbacks::MacroParsingBehavior {
        bindgen::callbacks::MacroParsingBehavior::Default
    }

    fn generated_name_override(
        &self,
        _item_info: bindgen::callbacks::ItemInfo<'_>,
    ) -> Option<String> {
        None
    }

    fn generated_link_name_override(
        &self,
        _item_info: bindgen::callbacks::ItemInfo<'_>,
    ) -> Option<String> {
        None
    }

    fn int_macro(&self, _name: &str, _value: i64) -> Option<bindgen::callbacks::IntKind> {
        None
    }

    fn str_macro(&self, _name: &str, _value: &[u8]) {}

    fn func_macro(&self, _name: &str, _value: &[&[u8]]) {}

    fn enum_variant_behavior(
        &self,
        _enum_name: Option<&str>,
        _original_variant_name: &str,
        _variant_value: bindgen::callbacks::EnumVariantValue,
    ) -> Option<bindgen::callbacks::EnumVariantCustomBehavior> {
        None
    }

    fn enum_variant_name(
        &self,
        _enum_name: Option<&str>,
        _original_variant_name: &str,
        _variant_value: bindgen::callbacks::EnumVariantValue,
    ) -> Option<String> {
        None
    }

    fn item_name(&self, _original_item_name: &str) -> Option<String> {
        None
    }

    fn include_file(&self, _filename: &str) {}

    fn read_env_var(&self, _key: &str) {}

    fn blocklisted_type_implements_trait(
        &self,
        _name: &str,
        _derive_trait: bindgen::callbacks::DeriveTrait,
    ) -> Option<bindgen::callbacks::ImplementsTrait> {
        None
    }

    fn add_derives(&self, _info: &bindgen::callbacks::DeriveInfo<'_>) -> Vec<String> {
        vec![]
    }

    /// Trims the comments found by clang.
    fn process_comment(&self, comment: &str) -> Option<String> {
        let mut comment = comment.trim().to_string();

        let finder = linkify::LinkFinder::new();
        let comment_links = comment.clone();
        let links = finder.links(comment_links.as_str()).collect::<Vec<_>>();
        for link in &links {
            comment.replace_range(link.start()..link.end(), &format!("<{}>", link.as_str()));
        }
        // let comment = comment.replace("\\n", "\n");
        let comment = comment.replace("[", r"`[");
        let comment = comment.replace("]", r"]`");
        // another approach
        // let comment = comment.replace("[", r"\[");
        // let comment = comment.replace("]", r"\]");
        Some(comment)
    }

    fn field_visibility(
        &self,
        _info: bindgen::callbacks::FieldInfo<'_>,
    ) -> Option<bindgen::FieldVisibilityKind> {
        None
    }
}
