extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/convert/converter.cc")
        .file("src/convert/converter_c.cc")
        .file("src/convert/capi.c")
        .cpp(true)
        .compile("libconv.a");
}
