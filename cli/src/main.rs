extern crate lindenmayer;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

mod model;

use lindenmayer::{png, svg};
fn main() {
    if !png::convert_save(&svg::draw_svg_utf8()) {
        println!("ERROR");
    }
}
