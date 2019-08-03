#[macro_use]
extern crate clap;

mod ffi;
mod xyz_parser;
mod text_parser;

fn main() {
    use clap::App;

    let yml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yml).get_matches();

    let ranks = value_t!(matches, "ranks", u32).unwrap_or(0);
    if ranks <= 0 {
        println!("unsupported ranks value.");
        return;
    }
    let format = matches.value_of("format").unwrap();
    if !(format == "xyz" || format == "text" || format == "db" || format == "def") {
        println!("unsupported format {}.", format);
        return;
    }
    let input = matches.value_of("input").unwrap();
    let output = matches.value_of("output").unwrap();

    mk_parse(format, ranks, input, output);
}

fn mk_parse(format: &str, ranks: u32, input: &str, output: &str) {
    match format {
        "xyz" => {
            ffi::parse(input, output, ranks, xyz_parser::new_parser(output));
        }
        "text" => {
            ffi::parse(input, output, ranks, text_parser::new_parser(output));
        }
        "db" => {}
        "def" => {}
        _ => unreachable!()
    }
}
