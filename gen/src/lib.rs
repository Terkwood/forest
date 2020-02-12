#[macro_use]
extern crate gdnative;
extern crate lindenmayer;

mod timed;

use gdnative::{ByteArray, Image, ImageTexture, Sprite};
use lindenmayer::{png, svg};
use timed::timed;

const PKG_NAME: &'static str = env!("CARGO_PKG_NAME");
const PKG_VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[allow(unused)]
const FOREST: &str = "FF-[-F+F+F]";

const IMG_WIDTH: i64 = 1682;
const IMG_HEIGHT: i64 = 2987;

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
    unsafe fn _ready(&self, mut owner: gdnative::Node) {
        let (svg_bytes, svg_time) = timed(|| svg::draw_svg_utf8());
        godot_print!(".. SVG generation in {:#?} ..", svg_time);

        let (png_bytes, png_time) = timed(|| png::convert_bytes(&svg_bytes));
        godot_print!("!! PNG conversion in {:#?} !!", png_time);
        let (_, godot_time) = timed(|| {
            let mut godot_bytes = ByteArray::new();
            for b in png_bytes {
                godot_bytes.push(b)
            }
            let mut image = Image::new();

            image.create_from_data(
                IMG_WIDTH,
                IMG_HEIGHT,
                false,
                Image::FORMAT_RGBA8,
                godot_bytes,
            );
            let mut image_texture = ImageTexture::new();
            image_texture.create_from_image(Some(image), 0);
            let mut sprite = Sprite::new();
            sprite.set_texture(Some(image_texture.to_texture()));
            owner.add_child(Some(sprite.to_node()), true)
        });
        godot_print!("## Godot image, texture, and sprite in {:#?}", godot_time)
    }
}

fn init(handle: gdnative::init::InitHandle) {
    handle.add_class::<DrawTree>();
    godot_print!("{:<8} {}", PKG_NAME, PKG_VERSION);
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();