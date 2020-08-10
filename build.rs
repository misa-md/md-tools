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
        Err(e) => panic!(format!("failed to execute command: {:?}\nerror: {}", cmd, e)),
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
