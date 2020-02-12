extern crate lindenmayer;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

mod model;

use crate::model::LSystem;
use std::fs;

const FILENAME: &str = "input.json";
fn main() {
    let input_file = fs::read_to_string(FILENAME).expect("could not read input");
    let parsed: Vec<LSystem> = serde_json::from_str(&input_file).expect("could not parse JSON");
    for ls in parsed {
        println!("We found {:#?}", ls)
    }
}
