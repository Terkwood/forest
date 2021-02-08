use crate::Size;
use tiny_skia::Pixmap;
use usvg::FitTo;

pub fn render_pixmap_bytes(svg_data: &[u8]) -> (Vec<u8>, Size) {
    let out = render_pixmap(svg_data);

    let size = Size {
        width: out.width(),
        height: out.height(),
    };

    (out.data().to_vec(), size)
}

fn render_pixmap(data: &[u8]) -> Pixmap {
    let rtree = usvg::Tree::from_data(data, &usvg::Options::default()).unwrap();

    let pixmap_size = rtree.svg_node().size.to_screen_size();
    let mut pixmap = Pixmap::new(pixmap_size.width() as u32, pixmap_size.height() as u32).unwrap();
    resvg::render(&rtree, FitTo::Original, pixmap.as_mut()).unwrap();

    pixmap
}

#[cfg(test)]
mod tests {
    use super::*;

    fn white_square_svg_bytes() -> Vec<u8> {
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
        svg_out
    }

    #[test]
    fn render_pixmap_bytes_preserves_color() {
        let (png_bytes, png_size) = render_pixmap_bytes(&white_square_svg_bytes());
        assert!(!png_bytes.is_empty());
        assert!(png_size.width > 0);
        assert!(png_size.height > 0);
        let mut found_white = false;
        let chunked_bytes = png_bytes.chunks(4);
        for four_bytes in chunked_bytes {
            if four_bytes == &[0, 0, 0, 0] {
                found_white = true;
                break;
            }
        }
        assert!(found_white)
    }
}
