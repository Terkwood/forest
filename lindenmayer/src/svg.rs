use crate::draw::*;
use crate::TreeOptions;

const INIT_DIRECTION: f32 = 0.0;

pub fn draw_svg_utf8(draw_props: TreeOptions) -> Vec<u8> {
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
