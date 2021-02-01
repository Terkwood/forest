mod parametric;
mod png;
mod svg;

mod draw;
mod lsys;

use crate::parametric::{PRule, PSym, PSystem};
use crate::parametric::{ParametricSymbol, ParametricSystem};
use expression::cond::Cond;
use expression_num::NumExpr as Expr;
use std::fmt::Debug;

pub struct TreeOptions {
    pub axiom: char,
    pub delta: f32,
    pub rules: Vec<Rule>,
    pub n: usize,
    pub stroke_width: f32,
    pub stroke_length: f32,
}

#[derive(Debug)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug)]
pub struct PngBytes {
    pub bytes: Vec<u8>,
    pub size: Size,
}

pub fn tree(opts: TreeOptions) -> PngBytes {
    let svg_bytes = svg::draw_svg_utf8(opts);
    let (bytes, size) = png::convert_svg_to_png_bytes(&svg_bytes);
    PngBytes { bytes, size }
}

fn develop_system(start: char, rules: Vec<Rule>, iter: usize) -> (Vec<PSym<char, f32>>, usize) {
    let mut system = System::new();
    for r in rules {
        system.add_rule(r)
    }
    system.develop(symstr(&start.to_string()), iter)
}

type Real = f32;
type SymExpr = PSym<char, Expr<Real>>;
type SymR = PSym<char, Real>;

pub type Rule = PRule<char, SymExpr, SymR, Cond<Expr<Real>>>;
type System = PSystem<Rule>;

pub fn symstr<S, R>(s: &str) -> Vec<S>
where
    S: ParametricSymbol<Sym = char, Param = R>,
    R: Clone + Debug + PartialEq,
{
    s.chars()
        .filter(|&c| !c.is_whitespace())
        .map(|c| S::new_from_vec(c, vec![]).unwrap())
        .collect()
}

pub fn rule(sym: char, successor: &str) -> Rule {
    Rule::new(sym, Cond::True, symstr(successor), 0)
}
