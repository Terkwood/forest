use lindenmayer::parametric::*;
use lindenmayer::*;
use serde_derive::Deserialize;
#[derive(Deserialize, Debug)]
pub struct Axiom(pub String);

#[derive(Deserialize, Debug)]
pub struct StepInput(pub String);

#[derive(Deserialize, Debug)]
pub struct StepOutput(pub String);

#[derive(Deserialize, Debug)]
pub struct Production {
    pub r#in: StepInput,
    pub out: StepOutput,
}

#[derive(Deserialize, Debug)]
pub struct LSystem {
    n: i32,
    d: f64,
    axiom: Axiom,
    rules: Vec<Production>,
    name: Option<String>,
}
impl LSystem {
    pub fn develop(&self) -> (Vec<PSym<char, f32>>, usize) {
        let axiom = symstr(&self.axiom.0);
        let mut system = create_system();
        for r in &self.rules {
            let a: Vec<char> = r.r#in.0.chars().collect();
            system.add_rule(rule(a[0], &r.out.0));
        }
        system.develop(axiom, self.n as usize)
    }
}
