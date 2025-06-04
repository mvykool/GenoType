use clap::{Arg, Command};

fn main() {
    let matches = Command::new("GenoType")
        .version("0.1.0")
        .author("mvyk0l")
        .about("convert rust types to TS")
        .arg(
            Arg::new("input")
                .short("i")
                .long("input")
                .required(true)
                .help("the rust file to process"),
        )
        .arg(
            Arg::new("output") 
                .short("o")
                .long("output")
                .required(true)
                .help("the name of the TS file to output"),
        )
        .get_matches();

    let input_filename = matches.get_one::<String>("input").expect("input required");
    let output_filename = matches.get_one::<String>("output").expect("output required");

    dbg!(input_filename);
    dbg!(output_filename);
}
