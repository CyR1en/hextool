# hextool
Simple command line tool for converting strings to hex and hex to string. Written in Rust!

This is my personal tool intended to be used for CTFs. I am aware that tools like `hex` and `unhex` already exist. 
However, I want to strengthen my Rust skills, therefore, I decided to make my own implementation and eventually distribute it to `crates.io`. 

#### Installation

For now the only way to install `hextool` command line tool is through `cargo`

```
cargo install hextool
```

#### Usage
```
Commandline tool to convert hex to string and string to hex

Usage: hextool <COMMAND>

Commands:
  hex    Change input to hex
  unhex  Change input from hex
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```



## hextool as lib

`hextool` could also be used as a library for your rust projects. You can do so by adding it to your `cargo.toml`

```toml
[dependencies]
hextool = { version = "version", default-features = false }
```

#### Usage

With `hextool` you can use `Hex` and `UnHex` to convert strings from and to hex strings.

##### Convert trait

Both `Hex` and `UnHex` implements this trait. This trait has a function called `convert` and it has the following parameters. It's important to know these parameters because it changes how the output.

```rust
fn convert(input: &str, numeric: bool, split: bool) -> String
```

- `input` - The input to convert
- `numeric` - If set to true, the input will be considered as a numeric value.
  - If the input value turns out to be a non-numeric value, the implementation will return a string containing a matching error message.
- `split` - If set to true, the output will be split in bytes.
  - For `Hex` this will separate the converted input to bytes. 
    - `aa` -> `61 61`
  - For `UnHex` this will separate the converted hex to a single ascii.
    - `6161` -> `a a`

##### Converting string to hex

```rust
use hextool::{Hex, Convert};

// Convert a string to hex
let hex = Hex::convert("hello", false, false);
println!("hello in hex: {}", hex); 
// #=> "hello in hex: 68656c6c6f"

// You can also split the output in bytes
let hex = Hex::convert("hello", false, true);
println!("hello in hex: {}", hex); 
// #=> "hello in hex: 68 65 6c 6c 6f"

// Convert a string with numeric flag. This will take the numerical value of the string.
// If the string is not a number, it will return an error.
let hex = Hex::convert("255", true, false);
println!("255 in hex: {}", hex); 
// #=> "255 in hex: {ff}"
```

##### Converting hex to string

```rust
use hextool::{UnHex, Convert};

// Convert a hex string to a string
let unhex = UnHex::convert("68656c6c6f", false, false);
println!("68656c6c6f in string: {}", unhex); 
// #=> "68656c6c6f in string: hello"

// Convert a hex string to a string with numeric flag. This will take the numerical value of the string.
let unhex = UnHex::convert("cafebabe", true, false);
println!("cafebabe in string: {}", unhex); 
// #=> "cafebabe in string: 3405691582"

// If numeric is set to false, only valid strings [0-9a-f] is accepted. If the string is not valid,
// it will return the string with the invalid string highlighted to red.
let unhex = UnHex::convert("aga", true, false);
println!("{}", unhex); 
// #=> "The highlighted chars can't be converted:\nag\u{1b}[31ma\u{1b}[0m."
```

