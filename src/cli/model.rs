use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CompiledFile {
    pub program: Program,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Program {
    pub data: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StarkNetGetCodeResponse {
    pub bytecode: Vec<String>,
}
