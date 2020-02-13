extern crate image;

mod boilerplate;

pub struct Shader(pub String);
#[derive(Debug)]
pub struct Err();
impl Shader {
    pub fn from(_png_rgba_bytes: &[u8]) -> Result<Shader, Err> {
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
    fn it_works() {
        let shader = Shader::from(&png_bytes()).expect("fail");
        let old_boilerplate = "m = Q(m,y,0,_,_,_,_,_,B,B,B)";
        assert_eq!(shader.0.contains(old_boilerplate), false);
    }
}
