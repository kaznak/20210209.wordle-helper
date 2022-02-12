use ansi_term::Colour;
use std::env;

use wordle_helper::hint;

fn main() {
    let tryhint =
        hint::TryHint::from_slice(env::args().skip(1).collect::<Vec<String>>().as_slice()).unwrap();
    eprintln!("{:?}", tryhint);
    println!(
        "This is {} in color, {} in color and {} in color",
        Colour::Red.paint("red"),
        Colour::Blue.paint("blue"),
        Colour::Green.paint("green")
    );
}
