#[macro_use]
extern crate clap;

mod ffi;
mod xyz_parser;
mod text_parser;

mod ans { pub mod voronoy; }

fn main() {
    use clap::App;

    let yml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yml)
        .setting(clap::AppSettings::ArgRequiredElseHelp)
        .version(crate_version!())
        .author(crate_authors!())
        .get_matches();

    if let Some(ref matches) = matches.subcommand_matches("conv") {
        parse_convert(matches);
        return;
    }
    if let Some(ref matches) = matches.subcommand_matches("ans") {
        parse_ans(matches);
        return;
    }
    println!("No subcommand is used");
}

fn parse_convert(matches: &&clap::ArgMatches) {
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

fn parse_ans(matches: &&clap::ArgMatches) {
    let input = matches.value_of("input").unwrap();
    let output = matches.value_of("output").unwrap();
    // todo method
    // todo, now only xyz format input is supported
    ans::voronoy::voronoy_ans(input, output)
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
