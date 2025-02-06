//! WFA bindings for rust using Bindgen.
//!
//! First, add `bindgen` to your dependencies:
//! ```toml
//! [build-dependencies]
//! bindgen = "0.60.1"
//! ```
//!
//! Then save this file as `build.rs` in the root of your crate.
//! Update the paths below to match your WFA installation/repository.
//!
//! The example has the WFA2-lib repository cloned in `../wfa2`.
//! Make sure to run `make lib_wfa` in the WFA repository.
//! The code below will tell cargo to link against `../wfa2/lib/libwfa.a`.
//!
//! The bindings will be writted to a special `OUT_DIR` set by cargo. See
//! `example.rs` for an example of how to include and use the generated
//! bindings.

extern crate bindgen;

use std::{env, fs, io, path::{Path, PathBuf}, process::Command};

fn wfa_build(wfa_dir: &str, out_dir: &Path) -> Result<(), io::Error> {

    let wfa_src = PathBuf::from(wfa_dir).canonicalize().unwrap();
    
    // Remove output directory if it exists
    if let Err(e) = fs::remove_dir_all(out_dir) {
        if e.kind() != io::ErrorKind::NotFound {
            return Err(e);
        }
    }

    // Create output directory
    fs::create_dir(out_dir)?;

    // Setup build
    Command::new("cmake")
        .arg(format!("{}", wfa_src.display()).as_str())
        .arg("-DCMAKE_BUILD_TYPE=Release")
        .current_dir(out_dir)
        .output()?;

    // And build
    Command::new("cmake")
        .arg("--build")
        .arg(".")
        .current_dir(out_dir)
        .output()?;

    Ok(())
}

fn wfa(wfa_dir: &str) {
    let bindings = bindgen::Builder::default()
        // Generate bindings for this header file.
        .header(format!("{wfa_dir}/wavefront/wavefront_align.h").as_str())
        // Add this directory to the include path to find included header files.
        .clang_arg(format!("-I{wfa_dir}").as_str())
        // Generate bindings for all functions starting with `wavefront_`.
        .allowlist_function("(wavefront_.*)|(cigar_.*)")
        // Generate bindings for all variables starting with `wavefront_`.
        .allowlist_var("(wavefront_.*)|(cigar_.*)")
        // Invalidate the built crate whenever any of the included header files
        // changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings_wfa.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings_wfa.rs"))
        .expect("Couldn't write bindings!");
}

fn main() -> Result<(), io::Error> {
    let wfa_dir = "WFA2-lib";
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out = out_dir.join(wfa_dir);

    // The directory of the WFA libraries, added to the search path.
    println!("cargo:rustc-link-search={}", out.display());
    // Link the `wfa-lib` library.
    println!("cargo:rustc-link-lib=static=wfa2");
    // Invalidate the built crate whenever the version changes
    println!("cargo:rerun-if-changed={wfa_dir}/VERSION.txt");

    wfa_build(wfa_dir, &out)?;

    wfa(wfa_dir);

    Ok(())
}
