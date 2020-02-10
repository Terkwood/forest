use resvg::prelude::*;

pub fn convert_bytes(data: &[u8]) -> Vec<u8> {
    convert(data).make_rgba_vec()
}

#[allow(unused)]
pub fn convert_save(data: &[u8]) -> bool {
    convert(data).save_png(std::path::Path::new("out.png"))
}

fn convert(data: &[u8]) -> std::boxed::Box<dyn resvg::OutputImage> {
    let rtree = usvg::Tree::from_data(data, &usvg::Options::default()).unwrap();
    let backend = resvg::default_backend();
    backend
        .render_to_image(&rtree, &Options::default())
        .unwrap()
}
