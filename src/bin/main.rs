use base64_rs::decoders::decode_faster;
use base64_rs::encoders::encode_faster;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Error: not enough arguments\n");
        std::process::exit(0);
    }

    let mut is_encoding = false;
    let mut from_file = false;
    let mut path_or_string = String::new();
    for arg in args {
        if arg == "-e" || arg == "--encode" {
            is_encoding = true;
        } else if arg == "-f" || arg == "--file" {
            from_file = true;
        } else {
            path_or_string = arg;
        }
    }

    if is_encoding {
        if from_file {
            let mut contents = fs::read(path_or_string).unwrap();
            println!("{}", encode_faster(&mut contents));
        } else {
            let string = String::from(path_or_string);
            let mut contents = string.into_bytes();
            println!("{}", encode_faster(&mut contents));
        }
    } else {
        fn remove_whitespace(s: &str) -> String {
            s.split_whitespace().collect()
        }

        if from_file {
            let string = fs::read_to_string(path_or_string).unwrap();
            let mut contents = String::from(remove_whitespace(&string)).into_bytes();
            println!("{}", decode_faster(&mut contents));
        } else {
            let string = path_or_string;
            let mut contents = String::from(remove_whitespace(&string)).into_bytes();
            println!("{}", decode_faster(&mut contents));
        }
    }
}
