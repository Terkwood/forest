extern crate shaders;
use shaders::Shader;
use std::fs::File;
use std::io::Read;
const IN_FILE: &str = "tiniest_in.png";
fn main() {
    let mut png_in = vec![];
    let mut file = File::open(IN_FILE).expect("cannot open file");
    file.read_to_end(&mut png_in).expect("cannot read");
    println!("{}", Shader::from(&png_in).expect("Failed").0)
}
