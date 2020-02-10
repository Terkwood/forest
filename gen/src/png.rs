use resvg::prelude::*;

pub fn convert(data: &[u8]) -> bool {
    let rtree = usvg::Tree::from_data(data, &usvg::Options::default()).unwrap();
    let backend = resvg::default_backend();
    let mut img = backend
        .render_to_image(&rtree, &Options::default())
        .unwrap();
    img.save_png(std::path::Path::new("out.png"))
}
