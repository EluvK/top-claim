use serde::Deserialize;
use std::{collections::HashMap, fs::File, io::Read, path::Path};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub groups: HashMap<String, GroupConfig>,
}

#[derive(Debug, Deserialize)]
pub struct GroupConfig {
    pub accounts: Vec<UserKeystoreAddrPubKey>,
    pub mining_pswd_enc: String, // mining_pswd
    pub topio_package_dir: String,
    pub topio_user: String,
    pub minimum_claim_value: u64,
    pub balance_target_address: String,
}

#[derive(Debug, Deserialize)]
pub struct UserKeystoreAddrPubKey {
    pub address: String,
    pub minerpubkey: String,
}
impl Config {
    /// Create ConfigJson object with config file path.
    pub fn read_from_file(file_path_str: &str) -> anyhow::Result<Self> {
        let content = read_file(file_path_str)?;
        Ok(serde_json::from_str(&content)?)
    }
}

pub fn read_file(file_path_str: &str) -> anyhow::Result<String> {
    let file_path = Path::new(file_path_str);
    let mut file = File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}
