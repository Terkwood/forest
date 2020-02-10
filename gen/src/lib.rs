#[macro_use]
extern crate gdnative;

pub mod lsys;
pub mod parametric;
mod svg;

const FOREST: &str = "FF-[-F+F+F]";

#[derive(gdnative::NativeClass)]
#[inherit(gdnative::Node)]
#[user_data(gdnative::user_data::ArcData<HelloWorld>)]
struct HelloWorld;

#[gdnative::methods]
impl HelloWorld {
    fn _init(_owner: gdnative::Node) -> Self {
        HelloWorld
    }

    #[export]
    fn _ready(&self, _owner: gdnative::Node) {
        let bytes_output = svg::draw_svg_utf8();
        godot_print!(
            "{}",
            String::from_utf8(bytes_output).unwrap_or("ERR".to_string())
        );
    }
}

fn init(handle: gdnative::init::InitHandle) {
    handle.add_class::<HelloWorld>();
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();
