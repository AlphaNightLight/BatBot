extern crate bindgen;

use std::env;
use std::path::PathBuf;

use bindgen::*;

fn main() {
    // This is the directory where the `c` library is located.
    let src_path = PathBuf::from("c-src")
        .canonicalize()
        .expect("cannot canonicalize path");

    // This is the path to the `c` headers file.
    let headers_path = src_path.join("wrapper.hpp");

    let lib_path = PathBuf::from("c-target").canonicalize().unwrap();
    //let headers_path_str = headers_path.to_str().expect("Path is not a valid string");

    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search={}", lib_path.to_str().unwrap());

    // Tell cargo to tell rustc to link our `hello` library. Cargo will
    // automatically know it must look for a `libhello.a` file.
    println!("cargo:rustc-link-lib=c-protocol");

    // Tell cargo to invalidate the built crate whenever the header changes.
    println!("cargo:rerun-if-changed={}", src_path.to_str().unwrap());
    let result = std::process::Command::new("make")
        .arg("-C")
        .arg(lib_path)
        .arg("all")
        .output()
        .expect("not possible to call make");
    if !result.status.success() {
        panic!("cannot build {}", String::from_utf8(result.stderr).unwrap())
    }

    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(headers_path.to_str().unwrap())
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(dir).join("src/generated/bindings.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}
