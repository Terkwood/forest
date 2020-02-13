use std::fs::File;

use crate::draw::*;
use crate::parametric::PSym;

const INIT_DIRECTION: f32 = 0.0;
const DEFAULT_ANGLE: f32 = 20.0;
const DEFAULT_DISTANCE: f32 = 4.0;

pub fn draw_svg_utf8(input: Vec<PSym<char, f32>>) -> Vec<u8> {
    let mut v = vec![];

    draw(
        &input,
        INIT_DIRECTION,
        DEFAULT_ANGLE,
        DEFAULT_DISTANCE,
        &mut v,
    );

    v
}

pub fn canned_draw_svg_utf8() -> Vec<u8> {
    let (after, _) = crate::develop_canned_system();

    let mut v = vec![];

    draw(
        &after,
        INIT_DIRECTION,
        DEFAULT_ANGLE,
        DEFAULT_DISTANCE,
        &mut v,
    );

    v
}

#[allow(unused)]
pub fn canned_draw_to_file() {
    let (after, iters) = crate::develop_canned_system();

    let mut file = File::create(&format!("plant_{:02}.svg", iters).to_string()).unwrap();
    draw(
        &after,
        INIT_DIRECTION,
        DEFAULT_ANGLE,
        DEFAULT_DISTANCE,
        &mut file,
    );
}
