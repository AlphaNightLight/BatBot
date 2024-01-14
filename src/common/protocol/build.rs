/*use std::env;
use std::path::PathBuf;

fn main() {
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    // Tell cargo to look for shared libraries in the specified directory
    //println!("cargo:rustc-link-search={}/c-src", dir);

    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    //println!("cargo:rustc-link-lib=bz2");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("./c-src/wrapper.hpp")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(format!("{}/src/generated", dir));//PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}*/

extern crate bindgen;

use std::env;
use std::path::PathBuf;

use bindgen::*;

/*/
fn compile(libdir_path: &PathBuf, name: &str){
    let compile = std::process::Command::new("clang")
        .arg("-c")
        .arg(libdir_path.join(format!("{name}.cpp")))
        //.arg(libdir_path.join("checker.cpp"))
        //.arg("-Oz")//test
        .arg("-o")
        .arg(libdir_path.join(format!("{name}.o")))

        .output()
        .expect("could not spawn `clang`");
    if !compile.status.success() {
        // Panic if the command was not successful.
        panic!(
            "could not compile object file stderr={}",
            String::from_utf8(compile.stderr).unwrap()
        );
    }
}*/

fn main() {
    // This is the directory where the `c` library is located.
    let libdir_path = PathBuf::from("c-src")
        // Canonicalize the path as `rustc-link-search` requires an absolute
        // path.
        .canonicalize()
        .expect("cannot canonicalize path");

    // This is the path to the `c` headers file.
    let headers_path = libdir_path.join("wrapper.hpp");
    let headers_path_str = headers_path.to_str().expect("Path is not a valid string");

    // This is the path to the intermediate object file for our library.
    //let obj_path = libdir_path.join("c-protocol.o");
    //let obj_path = libdir_path.join("c-checker.o");
    // This is the path to the static library file.
    //let lib_path = libdir_path.join("libc-protocol.a");

    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search={}", libdir_path.to_str().unwrap());

    // Tell cargo to tell rustc to link our `hello` library. Cargo will
    // automatically know it must look for a `libhello.a` file.
    println!("cargo:rustc-link-lib=c-protocol");

    // Tell cargo to invalidate the built crate whenever the header changes.
    println!("cargo:rerun-if-changed={}", libdir_path.to_str().unwrap());
    let result = std::process::Command::new("make")
        .arg("-C")
        .arg(libdir_path)
        .arg("all")
        .output()
        .expect("not possible to make");
    if !result.status.success() {
        panic!("cannot build {}", String::from_utf8(result.stderr).unwrap())
    }

    // Run `clang` to compile the `hello.c` file into a `hello.o` object file.
    // Unwrap if it is not possible to spawn the process.
    /*compile(&libdir_path, &obj_path);


    // Run `ar` to generate the `libhello.a` file from the `hello.o` file.
    // Unwrap if it is not possible to spawn the process.
    if !std::process::Command::new("ar")
        .arg("rcs")
        .arg(lib_path)
        .arg(obj_path)
        .output()
        .expect("could not spawn `ar`")
        .status
        .success()
    {
        // Panic if the command was not successful.
        panic!("could not emit library file");
    }*/

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(headers_path_str)
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
