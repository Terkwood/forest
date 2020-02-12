extern crate lindenmayer;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

mod model;

use crate::model::LSystem;
use lindenmayer::{png, svg};
use std::fs;

const FILENAME: &str = "input.json";
fn main() {
    let input_file = fs::read_to_string(FILENAME).expect("could not read input");
    let parsed: Vec<LSystem> = serde_json::from_str(&input_file).expect("could not parse JSON");
    for ls in parsed {
        let (s, _) = ls.develop();
        let svg_bytes = svg::draw_svg_utf8(s);
        let out_name = ls.name.unwrap_or("out".to_string());
        if png::convert_save(&svg_bytes, &out_name) {
            println!("Saved {}", out_name)
        } else {
            println!("ERR")
        }
    }
}
