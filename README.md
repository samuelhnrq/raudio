# raudio

My personal audio playground, just a bunch of scripts from the book (THE AUDIO
PROGRAMMING BOOK 978-0-262-01446-5) converted from C to Rust with my own personal spins.

## Building

All you need is rust & cargo, e.g. `cargo run` and `cargo build`

## Usage

I use the incredible and automagical library clap for argument processing, it also generates usage documentation, better than any doc I could write here that would become obsolete very fast

```
$ raudio --help
```

Most of the time it would be something along the lines of `raudio script_name param1 param2`
