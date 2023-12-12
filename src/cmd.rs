use std::{
    io::Write,
    process::{Command, Output},
};

use crate::claim::RewardInfoResp;

#[derive(Debug)]
pub struct TopioCommands {
    operator_user: String,
    exec_dir: String,
}

impl TopioCommands {
    pub fn new(user: &str, exec_dir: &str) -> Self {
        TopioCommands {
            operator_user: String::from(user),
            exec_dir: String::from(exec_dir),
        }
    }

    pub fn set_default_account(&self, address: &str, pswd: &str) -> anyhow::Result<Output> {
        let cmd_str = format!(
            r#"cd {} && topio wallet setDefaultAccount {}"#,
            &self.exec_dir, address
        );
        let mut command = Command::new("sudo")
            .args(["-u", &self.operator_user])
            .args(["sh", "-c"])
            .arg(cmd_str)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()?;

        let mut stdin = command.stdin.take().expect("Failed to use stdin");

        let pswd: String = pswd.into();
        std::thread::spawn(move || {
            stdin
                .write_all(pswd.as_bytes())
                .expect("Failed to write to stdin");
        });
        let output = command.wait_with_output()?;

        Ok(output)
    }

    // reward
    pub fn query_reward(&self, address: &str) -> anyhow::Result<RewardInfoResp> {
        let cmd_str = format!(
            r#"cd {} && topio mining queryMinerReward {} "#,
            &self.exec_dir, address
        );
        let c = Command::new("sudo")
            .args(["-u", &self.operator_user])
            .args(["sh", "-c"])
            .arg(cmd_str)
            .stdout(std::process::Stdio::piped())
            .spawn()?;

        let output = c.wait_with_output()?;
        Ok(serde_json::from_str(std::str::from_utf8(&output.stdout)?)?)
    }

    pub fn claim_reward(&self, address: &str, pswd: &str) -> anyhow::Result<Output> {
        _ = self.set_default_account(address, pswd)?;
        let cmd_str = format!(r#"cd {} && topio mining claimMinerReward"#, &self.exec_dir);
        let c = Command::new("sudo")
            .args(["-u", &self.operator_user])
            .args(["sh", "-c"])
            .arg(cmd_str)
            .stdout(std::process::Stdio::piped())
            .spawn()?;

        let output = c.wait_with_output()?;
        Ok(output)
    }

    pub fn get_balance(&self, address: &str, pswd: &str) -> anyhow::Result<u64> {
        _ = self.set_default_account(address, pswd)?;
        let cmd_str = String::from(
            r#"topio wallet listAccounts | head -n 5 | grep 'balance' | awk -F ' ' '{print $2}' "#,
        );
        let c = Command::new("sudo")
            .args(["-u", &self.operator_user])
            .args(["sh", "-c"])
            .arg(cmd_str)
            .stdout(std::process::Stdio::piped())
            .spawn()?;

        let output = c.wait_with_output()?;
        let v = std::str::from_utf8(&output.stdout)?
            .chars()
            .take_while(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse::<u64>()?;
        Ok(v)
    }

    pub fn transfer(&self, to_address: &str, amount: u64) -> anyhow::Result<Output> {
        let cmd_str = format!(
            r#"cd {} && topio transfer {} {}"#,
            &self.exec_dir, to_address, amount
        );
        let c = Command::new("sudo")
            .args(["-u", &self.operator_user])
            .args(["sh", "-c"])
            .arg(cmd_str)
            .stdout(std::process::Stdio::piped())
            .spawn()?;

        let output = c.wait_with_output()?;
        Ok(output)
    }
}
