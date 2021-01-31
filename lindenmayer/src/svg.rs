use crate::draw::*;
use crate::Rule;

const INIT_DIRECTION: f32 = 0.0;
const DEFAULT_ANGLE: f32 = 20.0;
const DEFAULT_DISTANCE: f32 = 4.0;

/*pub fn draw_svg_utf8(input: Vec<PSym<char, f32>>) -> Vec<u8> {
    let mut v = vec![];

    draw(
        &input,
        INIT_DIRECTION,
        DEFAULT_ANGLE,
        DEFAULT_DISTANCE,
        &mut v,
    );

    v
}*/

const RULE_X_FRIENDLY: &str = "F[+X]F[-X]+X";
const RULE_F_FRIENDLY: &str = "FF";

pub struct DrawProps {
    pub start: String,
    pub rules: Vec<Rule>,
    pub iter: usize,
}

pub fn draw_svg_utf8(draw_props: DrawProps) -> Vec<u8> {
    use crate::rule;
    let rules = vec![rule('X', RULE_X_FRIENDLY), rule('F', RULE_F_FRIENDLY)];

    let (after, _) = crate::develop_system(&draw_props.start, rules, draw_props.iter);

    let mut v = vec![];

    draw(
        &after,
        INIT_DIRECTION,
        DEFAULT_ANGLE,
        DEFAULT_DISTANCE,
        &mut v,
    );

    v
}

/*
#[allow(unused)]
pub fn canned_draw_to_file() {
    let (after, iters) = crate::develop_canned_system();

    let mut file = File::create(&format!("plant_{:02}.svg", iters).to_string()).unwrap();
    draw(
        &after,
        INIT_DIRECTION,
        DEFAULT_ANGLE,
        DEFAULT_DISTANCE,
        &mut file,
    );
}*/
