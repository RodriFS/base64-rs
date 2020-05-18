# base64-rs

A small cli base64 encoder and decoder

## How to run:

```
$ base64-rs -h

base64-rs 0.1
Rodrigo S. <rodrifs@gmail.com>
A small cli base64 encoder and decoder

USAGE:
    main [FLAGS] [OPTIONS] --encode --string <STRING>

FLAGS:
    -d, --decode     Decodes a given file or string
    -e, --encode     Encodes a given file or string
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --input <FILE>       Encode/Decode from a file, example: --input ./input.txt
    -o, --output <FILE>      Encode/Decode into a file, example: --output ./output.txt
    -s, --string <STRING>    Encode/Decode from a string, example: --string "hello"
```

## Examples:

Encoding from string:

```
$ base64-rs -e -s "hello"
aGVsbG8=
```

Decoding from string:

```
$ base64-rs -d -s "aGVsbG8="
hello
```

Encoding from file into a file:

```
$ base64-rs -e -i ./path/to/binary -o ./path/to/output.txt
```

Decoding from file into a file:

```
$ base64-rs -d -i ./path/to/file.txt -o ./path/to/output
```
