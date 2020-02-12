use crate::parametric::{PRule, PSym, PSystem};
use crate::parametric::{ParametricSymbol, ParametricSystem};
use expression::cond::Cond;
use expression_num::NumExpr as Expr;
use std::fmt::Debug;
use std::io::Write;
use turtle_graphics::{Canvas, Turtle};

#[allow(unused)]
const RULE_X_0: &str = "F-[[X]+X]+F[+FX]-X";
#[allow(unused)]
const RULE_X_1: &str = "F[+X][-X]FX";

#[allow(unused)]
const RULE_F_CLASSIC: &str = "FF-[-F+F+F]+[+F-F-F]";

const RULE_X_FRIENDLY: &str = "F[+X]F[-X]+X";
const RULE_F_FRIENDLY: &str = "FF";

const ITER: usize = 7;

pub fn develop_system() -> (Vec<PSym<char, f32>>, usize) {
    let axiom = symstr("X");

    let mut system = System::new();
    system.add_rule(rule('X', RULE_X_FRIENDLY));
    system.add_rule(rule('F', RULE_F_FRIENDLY));
    system.develop(axiom, ITER)
}

type Real = f32;
type SymExpr = PSym<char, Expr<Real>>;
type SymR = PSym<char, Real>;
type Rule = PRule<char, SymExpr, SymR, Cond<Expr<Real>>>;
type System = PSystem<Rule>;

pub fn draw<W: Write>(
    symstr: &[SymR],
    init_direction: f32,
    default_angle: f32,
    default_distance: f32,
    writer: &mut W,
) {
    let mut t = Canvas::new();
    t.right(init_direction);
    for sym in symstr.iter() {
        match (*sym.symbol(), sym.params().get(0)) {
            ('F', Some(&d)) => t.forward(d),
            ('F', None) => t.forward(default_distance),

            ('f', Some(&d)) => t.move_forward(d),
            ('f', None) => t.move_forward(default_distance),

            ('+', Some(&a)) => t.rotate(a),
            ('+', None) => t.rotate(default_angle),

            ('-', Some(&a)) => t.rotate(-a),
            ('-', None) => t.rotate(-default_angle),

            ('[', None) => t.push(),
            (']', None) => t.pop(),
            _ => {}
        }
    }
    t.save_svg(writer).unwrap();
}

#[allow(unused)]
fn symstr<S, R>(s: &str) -> Vec<S>
where
    S: ParametricSymbol<Sym = char, Param = R>,
    R: Clone + Debug + PartialEq,
{
    s.chars()
        .filter(|&c| !c.is_whitespace())
        .map(|c| S::new_from_vec(c, vec![]).unwrap())
        .collect()
}

#[allow(unused)]
fn rule(sym: char, successor: &str) -> Rule {
    Rule::new(sym, Cond::True, symstr(successor), 0)
}
