pub mod parametric;
pub mod png;
pub mod svg;

mod draw;
mod lsys;

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

//const ITER: usize = 7;

pub fn develop_system(start: &str, rules: Vec<Rule>, iter: usize) -> (Vec<PSym<char, f32>>, usize) {
    //    let axiom = symstr("X");

    let mut system = System::new();
    for r in rules {
        system.add_rule(r)
    }
    //    system.add_rule(rule('X', RULE_X_FRIENDLY));
    //    system.add_rule(rule('F', RULE_F_FRIENDLY));
    system.develop(symstr(start), iter)
}

pub type Real = f32;
pub type SymExpr = PSym<char, Expr<Real>>;
pub type SymR = PSym<char, Real>;

pub type Rule = PRule<char, SymExpr, SymR, Cond<Expr<Real>>>;
pub type System = PSystem<Rule>;
pub fn create_system() -> System {
    System::new()
}

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
