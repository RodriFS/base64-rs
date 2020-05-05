# base64-rs

A small cli encoder and decoder

## How to run:

As a string:

```
cargo run -- [Options] "string to encode"
```

As a file:

```
cargo run -- [Options] [-f, --file] [Path]
```

Options:

```
  -e --encode: Encodes the given string
  -d --decode: Decodes de given string (pending)
```

Path:
The path of the file you want to encode or decode (pending)

Optionally you can output your results to a txt file:

```
cargo run -- --encode "hello" > output.txt
```

```
cargo run -- --encode --file "input.txt" > output.txt
```
