extern crate gdnative;
extern crate lindenmayer;

mod timed;

use gdnative::api::*;
use gdnative::prelude::*;
use lindenmayer::{png, svg};
use timed::timed;

const PKG_NAME: &'static str = env!("CARGO_PKG_NAME");
const PKG_VERSION: &'static str = env!("CARGO_PKG_VERSION");

//
// some old rules
//
const _FOREST: &str = "FF-[-F+F+F]";

#[derive(NativeClass)]
#[inherit(Node)]
struct DrawTree {
    #[property(path = "base/start")]
    start: String,
    #[property(path = "base/rules")]
    rules: String,
    #[property(path = "base/iter")]
    iter: u64,
}

use lindenmayer::svg::DrawProps;

#[gdnative::methods]
impl DrawTree {
    fn new(_owner: &Node) -> Self {
        DrawTree {
            start: "".to_string(),
            rules: "".to_string(),
            iter: 0,
        }
    }

    #[export]
    fn _ready(&self, owner: &Node) {
        if self.start.is_empty() {
            godot_error!("NO START AXIOM DEFINED");
            return;
        }

        if let Some(start) = self.start.chars().nth(0) {
            if let Ok(rules) = parse_rules(&self.rules) {
                self.draw(start, rules, self.iter as usize, owner)
            }
        } else {
            godot_error!("NO START CHAR PROVIDED")
        }
    }

    fn draw(&self, start: char, rules: Vec<Rule>, iter: usize, owner: &Node) {
        let (svg_bytes, svg_time) = timed(|| svg::draw_svg_utf8(DrawProps { start, iter, rules }));
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

            let sprite = Sprite::new();
            sprite.set_texture(image_texture.upcast::<Texture>());

            owner.add_child(sprite.upcast::<Node>(), true)
        });
        godot_print!("## Godot image, texture, and sprite in {:#?}", godot_time)
    }
}

use lindenmayer::Rule;

#[derive(Debug)]
struct ParseErr;

/// Parse a rule system string written in a compact form, e.g.
///   "X:F[+X]F[-X]+X;F:FF"
fn parse_rules(compact: &str) -> Result<Vec<Rule>, ParseErr> {
    let semi_split = compact.split(';');
    let mut out = vec![];
    use lindenmayer::rule;
    for each in semi_split {
        if !each.is_empty() {
            let colon_split: Vec<&str> = each.split(':').collect();
            if colon_split.len() == 0 {
                continue;
            }
            if colon_split.len() != 2 {
                return Err(ParseErr);
            }
            let test = (colon_split[0].chars().nth(0), colon_split[1]);
            if let (Some(c), r) = test {
                out.push(rule(c, r))
            } else {
                return Err(ParseErr);
            }
        }
    }
    Ok(out)
}

fn init(handle: InitHandle) {
    handle.add_class::<DrawTree>();
    godot_print!("{:<8} {}", PKG_NAME, PKG_VERSION);
}

godot_init!(init);

#[cfg(test)]
mod tests {
    use crate::parse_rules;

    #[test]
    fn test_parse_rules() {
        const RULE_X_FRIENDLY: &str = "F[+X]F[-X]+X";
        const RULE_F_FRIENDLY: &str = "FF";

        let sample = &format!("X:{};F:{}", RULE_X_FRIENDLY, RULE_F_FRIENDLY);

        use lindenmayer::rule;
        let expected = vec![rule('X', RULE_X_FRIENDLY), rule('F', RULE_F_FRIENDLY)];
        let actual = parse_rules(sample).expect("parse");
        assert_eq!(actual, expected)
    }
}
