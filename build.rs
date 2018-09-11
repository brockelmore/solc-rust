extern crate bindgen;
extern crate cmake;

use cmake::Config;

use std::env;
use std::path::PathBuf;

fn main() {
    let dst = Config::new("solidity")
        .define("TESTS", "OFF")
        .define("TOOLS", "OFF")
        .build();
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-search=native={}/build/libsolc", dst.display());
    println!("cargo:rustc-link-search=native={}/build/libsolidity", dst.display());
    println!("cargo:rustc-link-search=native={}/build/libevmasm", dst.display());
    println!("cargo:rustc-link-search=native={}/build/libdevcore", dst.display());
    println!("cargo:rustc-link-search=native={}/build/deps/lib", dst.display());
    println!("cargo:rustc-link-lib=static=solc");
    println!("cargo:rustc-link-lib=static=solidity");
    println!("cargo:rustc-link-lib=static=evmasm");
    println!("cargo:rustc-link-lib=static=devcore");
    println!("cargo:rustc-link-lib=static=jsoncpp");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("solidity/libsolc/libsolc.h")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
