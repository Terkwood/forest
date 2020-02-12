use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Axiom(pub String);

#[derive(Deserialize, Debug)]
pub struct StepInput(pub String);

#[derive(Deserialize, Debug)]
pub struct StepOutput(pub String);

#[derive(Deserialize, Debug)]
pub struct Rule {
    pub r#in: StepInput,
    pub out: StepOutput,
}

#[derive(Deserialize, Debug)]
pub struct System {
    n: i32,
    d: f64,
    axiom: Axiom,
    rules: Vec<Rule>,
    name: Option<String>,
}
