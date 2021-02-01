use crate::draw::*;
use crate::Rule;

const INIT_DIRECTION: f32 = 0.0;

pub struct DrawOptions {
    pub axiom: char,
    pub delta: f32,
    pub rules: Vec<Rule>,
    pub n: usize,
    pub stroke_width: f32,
    pub stroke_length: f32,
}

pub fn draw_svg_utf8(draw_props: DrawOptions) -> Vec<u8> {
    let (after, _) = crate::develop_system(draw_props.axiom, draw_props.rules, draw_props.n);

    let mut v = vec![];

    draw(
        &after,
        INIT_DIRECTION,
        draw_props.delta,
        draw_props.stroke_length,
        draw_props.stroke_width,
        &mut v,
    );

    v
}
