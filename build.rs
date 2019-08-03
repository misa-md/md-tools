extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/convert/converter.c")
        .file("src/convert/capi.c")
        .cpp(false)
        .compile("libconv.a");
}
