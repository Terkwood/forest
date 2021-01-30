extern crate gdnative;
extern crate lindenmayer;

mod timed;

use gdnative::prelude::*;
use gdnative::api::*;
use lindenmayer::{png, svg};
use timed::timed;

const PKG_NAME: &'static str = env!("CARGO_PKG_NAME");
const PKG_VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[allow(unused)]
const FOREST: &str = "FF-[-F+F+F]";

#[derive(NativeClass)]
#[inherit(Node)]
#[user_data(user_data::ArcData<DrawTree>)]
struct DrawTree;

#[gdnative::methods]
impl DrawTree {
    fn new(_owner: &Node) -> Self {
        DrawTree
    }

    #[export]
    fn _ready(&self, owner: &Node) {
        let (svg_bytes, svg_time) = timed(|| svg::canned_draw_svg_utf8());
        godot_print!(".. SVG generation in {:#?} ..", svg_time);

        let ((png_bytes, size), png_time) = timed(|| png::convert_svg_to_png_bytes(&svg_bytes));
        godot_print!("!! PNG conversion in {:#?} !!", png_time);
        let (_, godot_time) = timed(|| {
            let mut godot_bytes = ByteArray::new();
            for b in png_bytes {
                godot_bytes.push(b)
            }
            let image = Image::new();

            image.create_from_data(
                size.width as i64,
                size.height as i64,
                false,
                Image::FORMAT_RGBA8,
                godot_bytes,
            );
            let image_texture = unsafe { ImageTexture::new().assume_unique() };
            image_texture.create_from_image(image, 0);
            godot_print!("# ... IMAGE TEXTURE");
            let sprite = Sprite::new();
            sprite.set_texture(image_texture.upcast::<Texture>());
            godot_print!("# ... SET TEXTURE");
            owner.add_child(sprite.upcast::<Node>(), true)
        });
        godot_print!("## Godot image, texture, and sprite in {:#?}", godot_time)
    }
}

fn init(handle: InitHandle) {
    handle.add_class::<DrawTree>();
    godot_print!("{:<8} {}", PKG_NAME, PKG_VERSION);
}

godot_init!(init);