use crate::cli::model::StarkNetGetCodeResponse;
use anyhow::{anyhow, Error, Result};
use execute::Execute;
use std::process::{Command, Stdio};

pub fn get_code(contract_address: &str) -> Result<StarkNetGetCodeResponse, Error> {
    //TODO: make network configurable through CLI
    let network = "alpha-goerli";
    let mut cmd = Command::new("starknet");
    cmd.arg("get_code");
    cmd.arg("--contract_address").arg(contract_address);
    cmd.arg("--network").arg(network);

    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    let output = cmd.execute_output().unwrap();

    if let Some(exit_code) = output.status.code() {
        if exit_code != 0 {
            return Err(anyhow!("starknet get_code failed"));
        }
    } else {
        return Err(anyhow!("starknet get_code interrupted"));
    }

    let output = String::from_utf8(output.stdout).unwrap();
    Ok(serde_json::from_str(&output)?)
}
