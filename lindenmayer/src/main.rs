extern crate lindenmayer;
use lindenmayer::{png, svg};
fn main() {
    if !png::convert_save(&svg::draw_svg_utf8()) {
        println!("ERROR");
    }
}
