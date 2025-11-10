use std::{env, path::PathBuf};

fn main() {
    // Set seach path to /usr/lib
    println!("cargo:rustc-link-search=/usr/lib");

    // Link hamlib
    println!("cargo:rustc-link-lib=hamlib");

    let bindings = bindgen::Builder::default()
        // use the wrapper header that includes all the hamlib headers
        .header("src/wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    let bindings_lists = bindgen::Builder::default()
        .header("src/wrapper_lists.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings for rig/amp/rotator lists");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
    bindings_lists
        .write_to_file(out_path.join("bindings_lists.rs"))
        .expect("Couldn't write lists bindings");
}