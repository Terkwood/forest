pub mod png;
pub mod svg;

mod draw;
mod lsys;
mod parametric;

use crate::parametric::{PRule, PSym, PSystem};
use crate::parametric::{ParametricSymbol, ParametricSystem};
use expression::cond::Cond;
use expression_num::NumExpr as Expr;
use std::fmt::Debug;

#[allow(unused)]
const RULE_X_0: &str = "F-[[X]+X]+F[+FX]-X";
#[allow(unused)]
const RULE_X_1: &str = "F[+X][-X]FX";

#[allow(unused)]
const RULE_F_CLASSIC: &str = "FF-[-F+F+F]+[+F-F-F]";

const RULE_X_FRIENDLY: &str = "F[+X]F[-X]+X";
const RULE_F_FRIENDLY: &str = "FF";

const ITER: usize = 7;

pub fn develop_canned_system() -> (Vec<PSym<char, f32>>, usize) {
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
