use brain_fuck::brain_fuck;
use std::env::args;

fn main() {
    let args: Vec<String> = args().collect();
    brain_fuck(&args[1])
}
