use serde_derive::Deserialize;
#[derive(Deserialize, Debug)]
pub struct Axiom(pub String);

#[derive(Deserialize, Debug)]
pub struct StepInput(pub char);

#[derive(Deserialize, Debug)]
pub struct StepOutput(pub String);

#[derive(Deserialize, Debug)]
pub struct Production {
    pub input: StepInput,
    pub output: StepOutput,
}

#[derive(Deserialize, Debug)]
pub struct LSystem {
    pub n: i32,
    pub d: f64,
    pub axiom: Axiom,
    pub prods: Vec<Production>,
    pub name: Option<String>,
}
