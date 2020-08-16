use spongemock::mock;
use std::env;

fn main() {
    if env::args().len() < 2 {
        eprintln!("Error: {}", mock("you didn't supply any input??"));
        std::process::exit(1);
    }

    for mock_val in env::args().skip(1) {
        println!("{}", mock(mock_val));
    }
}
