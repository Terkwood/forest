use crate::draw::*;
use crate::Rule;

const INIT_DIRECTION: f32 = 0.0;
const DEFAULT_ANGLE: f32 = 20.0;
const DEFAULT_DISTANCE: f32 = 4.0;

pub struct DrawProps {
    pub start: char,
    pub rules: Vec<Rule>,
    pub iter: usize,
}

pub fn draw_svg_utf8(draw_props: DrawProps) -> Vec<u8> {
    let (after, _) = crate::develop_system(draw_props.start, draw_props.rules, draw_props.iter);

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
