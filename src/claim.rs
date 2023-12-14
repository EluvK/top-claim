use serde::Deserialize;

use crate::config::Config;

#[derive(Debug, Deserialize)]
pub struct RewardInfo {
    pub accumulated: u64,
    pub accumulated_decimals: u64,
    pub issue_time: u64,
    pub last_claim_time: u64,
    pub unclaimed: u64,
    pub unclaimed_decimals: u64,
}

#[derive(Debug, Deserialize)]
pub struct RewardInfoResp {
    pub data: RewardInfo,
}

pub struct ClaimReward {
    config: Config,
}

impl ClaimReward {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        for group in self.config.groups.values() {
            let cmd = crate::cmd::TopioCommands::new(&group.topio_user, &group.topio_package_dir);
            let pswd = &group.mining_pswd_enc;
            for ac in &group.accounts {
                cmd.collect_reward(
                    &ac.address,
                    pswd,
                    group.minimum_claim_value,
                    &group.balance_target_address,
                )?;
            }
        }
        Ok(())
    }
}
