use rpassword;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ManGPTConfig {
    pub api_key: String,
    pub model: String,
    pub max_tokens: u16,
}

const FILE_NAME: &str = "mangpt-config.yml";
const DEFAULT_MODEL: &str = "gpt-4o-mini";
const DEFAULT_MAX_TOKENS: u16 = 1024u16;

pub fn load() -> Result<ManGPTConfig, Box<dyn std::error::Error>> {
    let config_dir = dirs::config_local_dir().ok_or("Can't find config directory")?;
    let config_file_path = config_dir.join(FILE_NAME);

    if !config_file_path.exists() {
        create_config_file(&config_file_path)?;
    }

    let content = fs::read_to_string(config_file_path)?;
    let config: ManGPTConfig = serde_yaml::from_str(&content)?;

    Ok(config)
}

fn create_config_file(
    config_file_path: &std::path::PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "You can create an OpenAI API-Key in your account https://platform.openai.com/api-keys"
    );
    let api_key = rpassword::prompt_password("Please Enter your OpenAI API-Key: ")?;

    let config = ManGPTConfig {
        api_key,
        model: DEFAULT_MODEL.to_string(),
        max_tokens: DEFAULT_MAX_TOKENS,
    };

    let config = serde_yaml::to_string(&config)?;
    fs::write(config_file_path, config)?;
    println!("Written config file {}", config_file_path.display());

    Ok(())
}
