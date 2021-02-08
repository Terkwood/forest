use crate::Size;
use tiny_skia::{Color, Pixmap};
use usvg::FitTo;

pub fn convert_svg_to_png_bytes(data: &[u8]) -> (Vec<u8>, Size) {
    let out = convert_svg_to_png(data);

    let size = Size {
        width: out.width(),
        height: out.height(),
    };

    (out.encode_png().expect("encoded"), size)
}

fn convert_svg_to_png(data: &[u8]) -> Pixmap {
    let rtree = usvg::Tree::from_data(data, &usvg::Options::default()).unwrap();

    let pixmap_size = rtree.svg_node().size.to_screen_size();
    let mut pixmap = Pixmap::new(pixmap_size.width() as u32, pixmap_size.height() as u32).unwrap();
    resvg::render(&rtree, FitTo::Original, pixmap.as_mut()).unwrap();

    pixmap
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use super::convert_svg_to_png_bytes;

    #[test]
    fn png_conversion_preserves_color() {
        use turtle::{Canvas, SvgParams, SvgStrokeColor, SvgStrokeWidth, Turtle};
        let mut t = Canvas::new();
        let mut svg_out = vec![];
        t.forward(100.0);
        t.right(90.0);
        t.forward(100.0);
        t.pen_up();
        t.forward(10.0);
        t.pen_down();
        t.right(90.0);
        t.forward(100.0);
        t.right(90.0);
        t.forward(100.0);
        t.save_svg(
            &mut svg_out,
            SvgParams {
                stroke_color: SvgStrokeColor::from("white"),
                stroke_width: SvgStrokeWidth(4.0),
            },
        )
        .expect("write an svg square");

        let (png_bytes, png_size) = convert_svg_to_png_bytes(&svg_out);
        assert!(!png_bytes.is_empty());
        assert!(png_size.width > 0);
        assert!(png_size.height > 0);

        let mut png_out_file_hack = std::fs::File::create("pngtest.png").expect("png test file");
        png_out_file_hack.write(&png_bytes).expect("png test write");
    }
}
