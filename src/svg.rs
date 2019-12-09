use crate::parametric::{PRule, PSym, PSystem};
pub use crate::parametric::{ParametricRule, ParametricSymbol, ParametricSystem};
pub use expression::cond::Cond;
pub use expression_num::NumExpr as Expr;
use std::fmt::Debug;
use std::fs::File;
use turtle_graphics::{Canvas, Turtle};

const RULE_X_0: &str = "F-[[X]+X]+F[+FX]-X";
const RULE_X_1: &str = "F[+X][-X]FX";

const RULE_F_CLASSIC: &str = "FF-[-F+F+F]+[+F-F-F]";

const RULE_X_FRIENDLY: &str = "F[+X]F[-X]+X";
const RULE_F_FRIENDLY: &str = "FF";

const ITER = 4;

pub fn fractal_plant() {
    let axiom = symstr("X");

    let mut system = System::new();
    system.add_rule(rule('X', RULE_X_FRIENDLY));
    system.add_rule(rule('F', RULE_F_FRIENDLY));
    println!("{:?}", system);

    let (after, iters) = system.develop(axiom, maxiter);

    draw(&after, 0.0, 20.0, 10.0, &format!("plant_{:02}", iters));
}

pub type Real = f32;
pub type SymExpr = PSym<char, Expr<Real>>;
pub type SymR = PSym<char, Real>;
pub type Rule = PRule<char, SymExpr, SymR, Cond<Expr<Real>>>;
pub type System = PSystem<Rule>;

pub fn draw(
    symstr: &[SymR],
    init_direction: f32,
    default_angle: f32,
    default_distance: f32,
    filename: &str,
) {
    let mut t = Canvas::new();
    t.right(init_direction);
    for sym in symstr.iter() {
        print!("{}", *sym.symbol());
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
    println!("");
    t.save_svg(&mut File::create(filename.to_string() + ".svg").unwrap())
        .unwrap();
}

#[allow(dead_code)]
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

#[allow(dead_code)]
pub fn rule(sym: char, successor: &str) -> Rule {
    Rule::new(sym, Cond::True, symstr(successor), 0)
}
