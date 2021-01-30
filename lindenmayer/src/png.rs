use usvg::FitTo;
use tiny_skia::Pixmap;

#[derive(Debug)]
pub struct Size{pub width: u32, pub height: u32}

pub fn convert_svg_to_png_bytes(data: &[u8]) -> (Vec<u8>, Size) {
    let out = convert_svg_to_png(data);
    
    let size = Size { width: out.width(), height: out.height()};

    (out.data().to_vec(), size)
}

fn convert_svg_to_png(data: &[u8]) -> Pixmap {
    let rtree = usvg::Tree::from_data(data, &usvg::Options::default()).unwrap();

    let pixmap_size = rtree.svg_node().size.to_screen_size();
    let mut pixmap =  Pixmap::new(pixmap_size.width() as u32, pixmap_size.height() as u32).unwrap();
    resvg::render(&rtree, FitTo::Original, pixmap.as_mut()).unwrap();

    pixmap
}
