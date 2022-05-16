use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub contract_address: String,
    pub contract_name: String,
    pub project_dir: String,
    pub build_dir: String,
}

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
