#[macro_use]
extern crate clap;

fn main() {
    use clap::App;

    let yml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yml).get_matches();

    let len = value_t!(matches, "ranks", u32).unwrap_or(0);
    if len <= 0 {
        println!("unsupported ranks value.");
        return;
    }
    let format = matches.value_of("format").unwrap();
    if !(format == "xyz" || format == "db" || format == "def") {
        println!("unsupported format {}.", format);
        return;
    }
    let input = matches.value_of("input").unwrap();
    let output = matches.value_of("output").unwrap();

}
