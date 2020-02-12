use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Axiom(pub String);

#[derive(Deserialize, Debug)]
pub struct StepInput(pub String);

#[derive(Deserialize, Debug)]
pub struct StepOutput(pub String);

#[derive(Deserialize, Debug)]
pub struct Rule {
    pub input: StepInput,
    pub output: StepOutput,
}

#[derive(Deserialize, Debug)]
pub struct System {
    iterations: i32,
    delta: f64,
    axiom: Axiom,
    productions: Vec<Rule>,
}
