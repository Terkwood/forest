use resvg::prelude::*;

pub fn convert_bytes(data: &[u8]) -> Vec<u8> {
    convert(data).make_rgba_vec()
}

pub fn convert_save(data: &[u8], name: &str) -> bool {
    convert(data).save_png(std::path::Path::new(&format!("lst_{}.png", name)))
}

fn convert(data: &[u8]) -> std::boxed::Box<dyn resvg::OutputImage> {
    let rtree = usvg::Tree::from_data(data, &usvg::Options::default()).unwrap();
    let backend = resvg::default_backend();
    backend
        .render_to_image(&rtree, &Options::default())
        .unwrap()
}
