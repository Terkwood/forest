use crate::parametric::ParametricSymbol;
use std::io::Write;
use turtle::{Canvas, SvgParams, SvgStrokeWidth, Turtle};

pub fn draw<W: Write>(
    symstr: &[crate::SymR],
    init_direction: f32,
    angle: f32,
    stroke_length: f32,
    stroke_width: f32,
    writer: &mut W,
) {
    let mut t = Canvas::new();
    t.right(init_direction);
    for sym in symstr.iter() {
        match (*sym.symbol(), sym.params().get(0)) {
            ('F', Some(&d)) => t.forward(d),
            ('F', None) => t.forward(stroke_length),

            ('f', Some(&d)) => t.move_forward(d),
            ('f', None) => t.move_forward(stroke_length),

            ('+', Some(&a)) => t.rotate(a),
            ('+', None) => t.rotate(angle),

            ('-', Some(&a)) => t.rotate(-a),
            ('-', None) => t.rotate(-angle),

            ('[', None) => t.push(),
            (']', None) => t.pop(),
            _ => {}
        }
    }

    t.save_svg(
        writer,
        SvgParams {
            stroke_width: SvgStrokeWidth(stroke_width),
            ..Default::default()
        },
    )
    .unwrap();
}
