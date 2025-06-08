use clap::{Arg, Command};
use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

fn main() {
    let matches = Command::new("GenoType")
        .version("0.1.0")
        .author("mvyk0l")
        .about("convert rust types to TS")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .required(true)
                .help("the rust file to process"),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .required(true)
                .help("the name of the TS file to output"),
        )
        .get_matches();

    let input_filename = matches.get_one::<String>("input").expect("input required");
    let output_filename = matches.get_one::<String>("output").expect("output required");

    dbg!(input_filename);
    dbg!(output_filename);

    let input_path = Path::new(input_filename);

    let mut input_file =
        File::open(input_path).expect(&format!("Unable to open file {}", input_path.display()));

    let mut input_file_text = String::new();

    input_file.read_to_string(&mut input_file_text).expect("unable to read file");

    //this is our tokenized version of Rust file ready to process
    let input_syntax: syn::File = syn::parse_file(&input_file_text).expect("unable to parse file");


    //this string will store the output of the typescript file that we will
    //continously append to as we process the Rust file
    let mut output_text = String::new();

    for item in input_syntax.items.iter() {
        match item {
            //this 'item type' enum variant matches our type alias
            syn::Item::Type(item_type) => {
                let type_text = parse_item_type(item_type);
                output_text.push_str(&type_text);
            }
            _ => {
                dbg!("encountered an unimplemented type");
            }
        }
    }

    let mut output_file = File::create(output_filename).unwrap();

    write!(output_file, "{}", output_text).expect("failed to write to output file");
}

fn parse_item_type(item_type: &syn::ItemType) -> String {
    String::from("todo")
}
