use base64_rs::decoders::decode_faster;
use base64_rs::encoders::encode_faster;
use clap::{App, Arg};
use std::fs::{write, File};
use std::io::Read;

fn main() {
    let matches = App::new("base64-rs")
        .version("0.1")
        .about("A small cli base64 encoder and decoder")
        .author("Rodrigo S. <rodrifs@gmail.com>")
        .arg(
            Arg::with_name("encode")
                .short("e")
                .long("encode")
                .help("Encodes a given file or string")
                .conflicts_with("decode")
                .required_unless("decode"),
        )
        .arg(
            Arg::with_name("decode")
                .short("d")
                .long("decode")
                .help("Decodes a given file or string")
                .conflicts_with("encode"),
        )
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .value_name("FILE")
                .help("Encode/Decode from a file, example: --input ./input.txt")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("FILE")
                .help("Encode/Decode into a file, example: --output ./output.txt")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("string")
                .short("s")
                .long("string")
                .empty_values(false)
                .value_name("STRING")
                .help("Encode/Decode from a string, example: --string \"hello\"")
                .takes_value(true)
                .required_unless("input")
                .conflicts_with("input"),
        )
        .get_matches();

    let is_encoding = matches.is_present("encode");
    let result;
    if matches.is_present("string") {
        let string = matches.value_of("string").unwrap();
        let mut contents = String::from(string).into_bytes();
        if is_encoding {
            result = encode_faster(&mut contents)
        } else {
            result = decode_faster(&mut contents)
        }
    } else if matches.is_present("input") {
        let path = matches.value_of("input").unwrap();
        let mut f = match File::open(path) {
            Ok(file) => file,
            Err(err) => {
                eprintln!("{}", err);
                std::process::exit(0);
            }
        };
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer).unwrap();

        if buffer.len() == 0 {
            eprintln!("Error: File is empty");
            std::process::exit(0);
        }
        if is_encoding {
            result = encode_faster(&mut buffer)
        } else {
            result = decode_faster(&mut buffer)
        }
    } else {
        eprintln!("Unexpected error");
        std::process::exit(0);
    }
    if matches.is_present("output") {
        write(matches.value_of("output").unwrap(), result).unwrap();
    } else {
        println!("{}", String::from_utf8(result).unwrap());
    }
}
