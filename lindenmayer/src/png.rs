use resvg::prelude::*;

pub fn convert_bytes(data: &[u8]) -> (Vec<u8>, Size) {
    let (mut out, size) = convert(data);
    
    (out.make_rgba_vec(), size)
}

pub fn convert_save(data: &[u8], name: &str) -> bool {
    convert(data).0.save_png(std::path::Path::new(&format!("lst_{}.png", name)))
}

pub struct Size{pub width: f64, pub height: f64}

fn convert(data: &[u8]) -> (std::boxed::Box<dyn resvg::OutputImage>, Size) {
    let rtree = usvg::Tree::from_data(data, &usvg::Options::default()).unwrap();

    let node = rtree.svg_node();
    let width = node.size.width();
    let height = node.size.height();
    
    let backend = resvg::default_backend();
    (backend
        .render_to_image(&rtree, &Options::default())
        .unwrap(), Size{width, height})
}
