use crate::parametric::ParametricSymbol;
use std::io::Write;
use turtle_graphics::{Canvas, Turtle};

pub fn draw<W: Write>(
    symstr: &[crate::SymR],
    init_direction: f32,
    default_angle: f32,
    default_distance: f32,
    writer: &mut W,
) {
    let mut t = Canvas::new();
    t.right(init_direction);
    for sym in symstr.iter() {
        match (*sym.symbol(), sym.params().get(0)) {
            ('F', Some(&d)) => t.forward(d),
            ('F', None) => t.forward(default_distance),

            ('f', Some(&d)) => t.move_forward(d),
            ('f', None) => t.move_forward(default_distance),

            ('+', Some(&a)) => t.rotate(a),
            ('+', None) => t.rotate(default_angle),

            ('-', Some(&a)) => t.rotate(-a),
            ('-', None) => t.rotate(-default_angle),

            ('[', None) => t.push(),
            (']', None) => t.pop(),
            _ => {}
        }
    }
    t.save_svg(writer).unwrap();
}
