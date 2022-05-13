use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CompiledFile {
    pub program: Program,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Program {
    pub data: Vec<String>,
}
