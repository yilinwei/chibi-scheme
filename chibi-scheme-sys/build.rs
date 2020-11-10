extern crate bindgen;

use std::collections::HashSet;
use std::env;
use std::io;
use std::path::Path;
use std::process::Command;

fn make(dir: &Path) -> io::Result<()> {
    Command::new("make")
        .current_dir(dir.clone().join("chibi-scheme"))
        .arg("clean")
        .status()?;

    Command::new("make")
        .current_dir(dir.clone().join("chibi-scheme"))
        .arg(format!("PREFIX={}/install", dir.clone().to_str().unwrap()))
        .arg("uninstall")
        .status()?;

    Command::new("make")
        .current_dir(dir.clone().join("chibi-scheme"))
        .arg(format!("PREFIX={}/install", dir.clone().to_str().unwrap()))
        .status()?;

    let status = Command::new("make")
        .current_dir(dir.clone().join("chibi-scheme"))
        .arg(format!("PREFIX={}/install", dir.clone().to_str().unwrap()))
        .arg("install")
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "Failed to run make"))
    }
}

// Taken from https://github.com/rust-lang/rust-bindgen/issues/687
#[derive(Debug)]
struct IgnoreMacros(HashSet<String>);

impl bindgen::callbacks::ParseCallbacks for IgnoreMacros {
    fn will_parse_macro(&self, name: &str) -> bindgen::callbacks::MacroParsingBehavior {
        if self.0.contains(name) {
            bindgen::callbacks::MacroParsingBehavior::Ignore
        } else {
            bindgen::callbacks::MacroParsingBehavior::Default
        }
    }
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    make(Path::new(&manifest_dir)).unwrap();
    let ignored_macros = IgnoreMacros(
        vec![
            "FP_INFINITE".into(),
            "FP_NAN".into(),
            "FP_NORMAL".into(),
            "FP_SUBNORMAL".into(),
            "FP_ZERO".into(),
            "IPPORT_RESERVED".into(),
            "FP_INT_UPWARD".into(),
            "FP_INT_DOWNWARD".into(),
            "FP_INT_TOWARDZERO".into(),
            "FP_INT_TONEARESTFROMZERO".into(),
            "FP_INT_TONEAREST".into(),
        ].into_iter()
        .collect(),
    );
    let bindings = bindgen::Builder::default()
        .header(format!("{}/chibi-scheme/include/chibi/eval.h", &manifest_dir))
        .clang_arg(format!("-I/{}/chibi-scheme/include", &manifest_dir))
        .parse_callbacks(Box::new(ignored_macros))
        .generate()
        .expect("Unable to generate bindings");
    bindings
        .write_to_file(format!("{}/bindings.rs", out_dir))
        .expect("Could not write bindings");

    println!("cargo:rustc-link-search=native={}/chibi-scheme/", &manifest_dir);
    println!("cargo:rustc-link-lib=chibi-scheme");
}
