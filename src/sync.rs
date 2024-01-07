use anyhow::Ok;
use reqwest::blocking::Client;
use serde::Deserialize;
use std::{fs, path::Path, sync::OnceLock, time::Duration};

fn get_client() -> &'static Client {
    static INSTANCE: OnceLock<Client> = OnceLock::new();
    INSTANCE.get_or_init(Client::new)
}

#[derive(Debug, Deserialize)]
pub struct Config {
    url: String,
    target_path: String,
    update_interval_seconds: u64,
}

impl Config {
    pub fn new(file: &str) -> anyhow::Result<Config> {
        let content = fs::read_to_string(file)?;
        let config: Config = toml::from_str(content.as_str())?;
        Ok(config)
    }

    pub fn get_update_duration(&self) -> Duration {
        Duration::from_secs(self.update_interval_seconds)
    }

    fn pull_from_remote(&self) -> anyhow::Result<String> {
        let respone = get_client().get(&self.url).send()?;
        let text = respone.text()?;
        Ok(text)
    }

    fn get_local_file(&self) -> anyhow::Result<String> {
        let content = fs::read_to_string(Path::new(self.target_path.as_str()))?;
        Ok(content)
    }

    pub fn sync(&self) -> anyhow::Result<()> {
        log::warn!("start to sync");
        let left = self.pull_from_remote()?;
        let right = self.get_local_file()?;
        if left != right {
            fs::write(self.target_path.as_str(), left)?;
            log::info!("sync finish")
        } else {
            log::info!("no need to sync")
        }
        Ok(())
    }
}
