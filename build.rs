extern crate cc;
extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    cc::Build::new()
        .file("src/convert/converter.c")
        .file("src/convert/capi.c")
        .cpp(false)
        .compile("libconv.a");

    if cfg!(feature = "minio-analysis") {
        gen_minio_rw_api();
    }
    gen_conversion_api();
}

fn gen_minio_rw_api() {
    println!("cargo:rerun-if-changed=src/ans/minio/minio-rw.go");

    // run `go build --buildmode=c-archive -o /path/to/save/libminio_rw.a`
    let lib_out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let mut cmd = Command::new("go");
    cmd.current_dir("src/ans/minio")
        .envs(env::vars())
        .args(&["build", "--buildmode=c-archive", "-o", format!("{}/{}", lib_out_path.display(), "libminio_rw.a").as_str()]);
    // .expect("failed to execute command `go build`");

    let status = match cmd.status() {
        Ok(status) => status,
        Err(e) => panic!("failed to execute command: {:?}\nerror: {}", cmd, e),
    };
    assert!(status.success());

    println!("cargo:rustc-link-search={}", lib_out_path.display());
    println!("cargo:rustc-link-lib=static=minio_rw");

    // Configure and generate bindings.
    let bindings = bindgen::Builder::default()
        .header(format!("{}/{}", lib_out_path.display(), "libminio_rw.h").as_str())
        .generate()
        .expect("unable to generate bindings");

    // Write the generated bindings to an output file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("minio_rw_bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn gen_conversion_api() {
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=src/convert/capi.h");

    // The bindgen::Builder is the main entry point to bindgen,
    // and lets you build up options for the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("src/convert/capi.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("capi_bindings.rs"))
        .expect("Couldn't write bindings!");
}
