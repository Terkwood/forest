extern crate image;

mod boilerplate;

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
        Ok(Shader(boilerplate::BOGUS.to_string()))
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
        let old_boilerplate = "m = Q(m,y,0,_,_,_,_,_,B,B,B)";
        assert!(!shader.0.contains(old_boilerplate));
    }
}
