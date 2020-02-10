#[macro_use]
extern crate gdnative;

pub mod lsys;
pub mod parametric;
mod png;
mod svg;
mod timed;

use timed::timed;

const PKG_NAME: &'static str = env!("CARGO_PKG_NAME");
const PKG_VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[allow(unused)]
const FOREST: &str = "FF-[-F+F+F]";

#[derive(gdnative::NativeClass)]
#[inherit(gdnative::Node)]
#[user_data(gdnative::user_data::ArcData<DrawTree>)]
struct DrawTree;

#[gdnative::methods]
impl DrawTree {
    fn _init(_owner: gdnative::Node) -> Self {
        DrawTree
    }

    #[export]
    unsafe fn _ready(&self, _owner: gdnative::Node) {
        let bytes_output = svg::draw_svg_utf8();

        let (_bytes, convert_time) = timed(|| png::convert_bytes(&bytes_output));
        godot_print!("!! PNG conversion succeeded in {:#?} !!", convert_time)
    }
}

fn init(handle: gdnative::init::InitHandle) {
    handle.add_class::<DrawTree>();
    godot_print!("{:<8} {}", PKG_NAME, PKG_VERSION);
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();
