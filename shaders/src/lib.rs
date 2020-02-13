extern crate image;

mod boilerplate;

use crate::image::GenericImageView;

pub struct Shader(pub String);
#[derive(Debug)]
pub enum Err {
    PngLoad,
}
impl From<image::ImageError> for Err {
    fn from(_: image::ImageError) -> Err {
        Err::PngLoad
    }
}
impl Shader {
    pub fn from(png_rgba_bytes: &[u8]) -> Result<Shader, Err> {
        let png_img = image::load_from_memory(png_rgba_bytes)?;
        let (_width, _height) = (png_img.width(), png_img.height());
        Ok(Shader(format!(
            "{}{}",
            boilerplate::BEGINNING,
            boilerplate::BOGUS_REST
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::Shader;
    use std::fs::File;
    use std::io::Read;
    const TEST_FILE_IN: &str = "in.png";
    fn png_bytes() -> Vec<u8> {
        let mut png_in = vec![];
        let mut file = File::open(TEST_FILE_IN).expect("cannot open file");
        file.read_to_end(&mut png_in).expect("cannot read");
        png_in
    }

    #[test]
    fn bogus_image_bytes_blow_up() {
        let shader = Shader::from(&[0, 1, 100, 101]);
        assert!(if let Ok(_) = shader { false } else { true })
    }
    #[test]
    fn never_generate_antiquated_mushroom_stuff() {
        let shader = Shader::from(&png_bytes()).expect("fail");
        let old_boilerplate = vec![
            "float Q(float m, int y, int i, float a, float b, float c, float d, float e, float f, float g, float h) {",
            "it = mushroom(uv - vec2(.1, .1), vec2(20.));",
            "vec3 mushroom(vec2 p, vec2 scale) {",
            "m = Q(m,y,0,_,_,_,_,_,B,B,B)",
            "return (a+4.*(b+4.*(c+4.*(d+4.*(e+4.*(f+4.*(g+4.*(h+4.))))))));",
        ];
        for o in old_boilerplate {
            assert!(!shader.0.contains(o));
        }
    }
}
