extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::process::Command;

use bindgen::CargoCallbacks;

static EXTERNAL_LIB_NAME: &str = "fake6502";

fn main() {
    fake6502_bindgen();
}

fn fake6502_bindgen() {
    // This is the directory where the `c` library is located.
    let libdir_path = PathBuf::from(format!("../../{EXTERNAL_LIB_NAME}"))
        // Canonicalize the path as `rustc-link-search` requires an absolute
        // path.
        .canonicalize()
        .expect("cannot canonicalize path");

    // This is the path to the `c` headers file.
    let headers_path = libdir_path.join(format!("{EXTERNAL_LIB_NAME}.h"));
    let headers_path_str = headers_path.to_str().expect("Path is not a valid string");

    // This is the path to the intermediate object file for our library.
    let obj_path = libdir_path.join(format!("{EXTERNAL_LIB_NAME}.o"));
    // This is the path to the static library file.
    let lib_path = libdir_path.join(format!("lib{EXTERNAL_LIB_NAME}.a"));

    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search={}", libdir_path.to_str().unwrap());

    // Tell cargo to tell rustc to link our `hello` library. Cargo will
    // automatically know it must look for a `libhello.a` file.
    println!("cargo:rustc-link-lib={EXTERNAL_LIB_NAME}");

    // Tell cargo to invalidate the built crate whenever the header changes.
    println!("cargo:rerun-if-changed={}", headers_path_str);

    // Run `clang` to compile the `hello.c` file into a `hello.o` object file.
    // Unwrap if it is not possible to spawn the process.
    if !Command::new("clang")
        .arg("-c")
        .arg("-o")
        .arg(&obj_path)
        .arg(libdir_path.join(format!("{EXTERNAL_LIB_NAME}.c")))
        .arg("-D")
        .arg("CMOS6502")
        .arg("-D")
        .arg("DECIMALMODE")
        .output()
        .expect("could not spawn `clang`")
        .status
        .success()
    {
        // Panic if the command was not successful.
        panic!("could not compile object file");
    }

    // Run `ar` to generate the `lib{EXTERNAL_NAME}.a` file from the `hello.o` file.
    // Unwrap if it is not possible to spawn the process.
    if !Command::new("ar")
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
    }

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .allowlist_function("fake6502_reset")
        .allowlist_function("fake6502_step")
        .allowlist_function("fake6502_irq")
        .allowlist_function("fake6502_nmi")
        .no_copy("fake6502_context")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}
