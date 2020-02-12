use std::fs::File;

use crate::draw::*;

pub fn draw_svg_utf8() -> Vec<u8> {
    let (after, _) = crate::develop_canned_system();

    let mut v = vec![];

    draw(&after, 0.0, 20.0, 10.0, &mut v);

    v
}

#[allow(unused)]
pub fn draw_to_file() {
    let (after, iters) = crate::develop_canned_system();

    let mut file = File::create(&format!("plant_{:02}.svg", iters).to_string()).unwrap();
    draw(&after, 0.0, 20.0, 10.0, &mut file);
}
