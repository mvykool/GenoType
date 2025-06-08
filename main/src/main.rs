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
            // enum variants
            syn::Item::Type(item_enum) => {
                let enum_text = parse_item_type(item_enum);
                output_text.push_str(&enum_text);
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
    let mut output_text = String::new();

    output_text.push_str("export type");

    // 'indent' is the name of the type alias
    output_text.push_str(&item_type.ident.to_string());
    output_text.push_str(" = ");

    let type_string = parse_type(&item_type.ty);
    output_text.push_str(&type_string);
    output_text.push_str(";");

    output_text
}

fn parse_type(syn_type: &syn::Type) -> String {
    let mut output_text = String::new();

    match syn_type {
        //primitive types like i32 will match path
        //we curretnly do not do anything with full paths
        //so we take only the last() segment aka the type name
        syn::Type::Path(type_path) => {
            let segment = type_path.path.segments.last().unwrap();

            let field_type = segment.ident.to_string();

            let ts_field_type = parse_type_ident(&field_type).to_owned();
            output_text.push_str(&ts_field_type);

            match &segment.arguments {
                // a simple type like i32 matches here as it
                // does not include any arguments
                syn::PathArguments::None => {}
                _ => {
                    dbg!("encountered an unimplemented token");
                }
            }
        }

        _ => {
            dbg!("encountered an unimplemented token");
        }
    };

    output_text
}

fn parse_type_ident(ident: &str) -> &str {
    match ident {
        "i8" | "i16" | "i32" | "i64" | "i128" | "u8" | "u16" | "u32" | "u64" | "f32" | "f64"
        | "isize" | "usize" => "number",
        "str" | "String" | "char" => "string",
        _=> ident,
    }
}

fn parse_item_enum(item_enum: &syn::ItemEnum) -> String {
    let mut output_text = String::new();

    output_text.push_str("export type");
    output_text.push_str(" ");

    let enum_name = item_enum.ident.to_string();
    output_text.push_str(" ");
    output_text.push_str("=");
    output_text.push_str(" ");

    for variant in item_enum.variants.iter() {
        // use the pipe character for union types
        // typescript also allows it before the first type as valid syntax
        output_text.push_str(" | {");
        output_text.push_str(" ");

        //for simplicity this implementation we are using assumes that enum will be
        //using serfe's adjacently tagged attribute
        //serde tag = "t", content = "c"
        //as an improvement on this implementation you could parse the attribute
        //and habdle the enum differently depending on which attribute the user chose
        output_text.push_str("t: \"");
        let variant_name = variant.ident.to_string();
        output_text.push_str(&variant_name);
        output_text.push_str("\" , c: ");

        match &variant.fields {
            syn::Fields::Named(named_fields) => {
                output_text.push_str("{");
                for field in named_fields.named.iter() {
                    if let Some(ident) = &field.ident {
                        output_text.push_str(&ident.to_string());
                        output_text.push_str(":");

                        let field_type = parse_type(&field.ty);
                        output_text.push_str(&field_type);
                        output_text.push_str(";");
                    }
                }
                outout_text.push_str("}");
            }
            syn::Fields::Unnamed(unnamed_fields) => {
                //currently only support a single unnamed field
                let unnamed_field = unnamed_fields.unnamed.first().unwrap();
                let field_type = parse_type(&unnamed_field.ty);
                output_text.push_str(&field_type);
            }
            syn::Fields::Unit => {
                output_text.push_str("undefined");
            }
        }

        output_text.push_str("}");
    }
    output_text.push_str(";");

    output_text
}
