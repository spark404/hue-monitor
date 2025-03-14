use duration_string::DurationString;
use serde::Deserialize;
use std::env::var;
use std::error::Error;
use std::path::PathBuf;
use std::time::Duration;

#[derive(Deserialize)]
pub struct Config {
    #[serde(default = "default_duration")]
    pub interval: DurationString,
    pub huebridge: BridgeConfig,
    pub telegraf: TelegrafConfig,
}

#[derive(Deserialize)]
pub struct BridgeConfig {
    pub url: String,
    pub token: String,
    pub certificate: Option<PathBuf>,
}

#[derive(Deserialize)]
pub struct TelegrafConfig {
    pub endpoint: String,
}

fn default_duration() -> DurationString {
    Duration::from_secs(5).into()
}

pub fn parse_config(toml: String) -> Result<Config, Box<dyn Error>> {
    let mut config: Config = toml::from_str(&toml)?;

    // token could be a value or a string pointing to an env var
    if let Some(env_var) = config.huebridge.token.strip_prefix("$") {
        if let Ok(val) = var(env_var) {
            config.huebridge.token = String::from(val);
        };
    };

    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::set_var;

    #[test]
    fn test_parse_config() {
        let toml = r#"interval = "10s"

[huebridge]
url = "https://192.168.0.1"
token = "token"

[telegraf]
endpoint = "tcp://192.168.0.1:8094""#;

        let config = parse_config(toml.parse().unwrap());
        assert!(config.is_ok());

        let config = config.unwrap();
        assert_eq!(config.huebridge.url, "https://192.168.0.1");
        assert_eq!(config.huebridge.token, "token");
        assert_eq!(config.huebridge.certificate, None);

        assert_eq!(config.telegraf.endpoint, "tcp://192.168.0.1:8094");

        assert_eq!(config.interval.as_secs(), 10);
    }

    #[test]
    fn test_parse_config_with_certificate() {
        let toml = r#"[huebridge]
url = "https://192.168.0.1"
token = "token"
certificate = "huecert.pem"

[telegraf]
endpoint = "tcp://192.168.0.1:8094""#;

        let config = parse_config(toml.parse().unwrap());
        assert!(config.is_ok());

        let config = config.unwrap();
        assert_eq!(config.huebridge.url, "https://192.168.0.1");
        assert_eq!(config.huebridge.token, "token");
        assert_eq!(config.huebridge.certificate, Some(PathBuf::from("huecert.pem")));

        assert_eq!(config.telegraf.endpoint, "tcp://192.168.0.1:8094");

        assert_eq!(config.interval.as_secs(), 5);
    }

    #[test]
    fn test_parse_config_with_env_token() {
        let toml = r#"[huebridge]
url = "https://192.168.0.1"
token = "$TEST_TOKEN"
certificate = "huecert.pem"

[telegraf]
endpoint = "tcp://192.168.0.1:8094""#;
        set_var("TEST_TOKEN", "test-token");

        let config = parse_config(toml.parse().unwrap());
        assert!(config.is_ok());

        let config = config.unwrap();
        assert_eq!(config.huebridge.url, "https://192.168.0.1");
        assert_eq!(config.huebridge.token, "test-token");
        assert_eq!(config.huebridge.certificate, Some(PathBuf::from("huecert.pem")));

        assert_eq!(config.telegraf.endpoint, "tcp://192.168.0.1:8094");

        assert_eq!(config.interval.as_secs(), 5);
    }

    #[test]
    fn test_parse_config_with_missing_token() {
        let toml = r#"[huebridge]
url = "https://192.168.0.1"
certificate = "huecert.pem"

[telegraf]
endpoint = "tcp://192.168.0.1:8094""#;

        let config = parse_config(toml.parse().unwrap());
        assert!(config.is_err());
    }
}