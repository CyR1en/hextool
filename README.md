# hextool
Simple command line tool for converting strings to hex and hex to string. Written in Rust!

This is my personal tool intended to be used for CTFs. I am aware that tools like `hex` and `unhex` already exist. 
However, I want to strengthen my Rust skills, therefore, I decided to make my own implementation and eventually distribute it to `crates.io`. 

#### Usage
```
Commandline tool to convert hex to string and string to hex

Usage: hextool.exe <COMMAND>

Commands:
  hex    Change input to hex
  unhex  Change input from hex
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```
