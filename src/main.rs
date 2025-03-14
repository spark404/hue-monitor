mod hue;

use crate::hue::client::{Client, ClientConfig};
use clap::Parser;
use serde::ser::StdError;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;
use std::{env, fs};
use telegraf::{Client as TelegrafClient, Metric};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(
        short,
        long,
        value_name = "FILE",
        default_value = "/etc/hue_monitor.conf"
    )]
    config: PathBuf,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
}

#[derive(Deserialize)]
struct Config {
    huebridge: BridgeConfig,
    telegraf: TelegrafConfig,
}

#[derive(Deserialize)]
struct BridgeConfig {
    url: String,
    token: String,
    certificate: Option<PathBuf>,
}

#[derive(Deserialize)]
struct TelegrafConfig {
    endpoint: String,
}

#[derive(Metric)]
#[measurement = "temperature"]
struct Temperature {
    temperature: f64,
    #[telegraf(tag)]
    device: String,
    #[telegraf(tag)]
    device_id: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    println!("Starting hue monitor...");
    println!("Config: {:?}", cli.config);

    let current_directory = env::current_dir()?;

    // Print the current working directory
    println!("Current working directory: {:?}", current_directory);

    let toml_content = fs::read_to_string(cli.config)?;
    let parsed_toml: Config = toml::from_str(&toml_content)?;
    println!("Configured hub: {:?}", parsed_toml.huebridge.url);

    let client = Client::new(ClientConfig {
        url: parsed_toml.huebridge.url,
        token: parsed_toml.huebridge.token,
        certificate: parsed_toml.huebridge.certificate,
    })?;

    let telegrafClient = TelegrafClient::new(&parsed_toml.telegraf.endpoint);
    if let Err(e) = telegrafClient {
        panic!("Failed to create Telegraf client: {:?}", e);
    }
    let mut telegrafClient = telegrafClient.unwrap();

    let response = client.send(hue::requests::Config {})?;
    println!("Response: {:?}", response);

    let device_list = client.send(hue::requests::DevicesRequest {})?;
    let mut device_names: HashMap<String, String> = HashMap::new();
    let mut temperature_devices: Vec<String> = vec!();
    for x in device_list.data {
        device_names.insert(x.id, x.metadata.name);

        let mut iter = x.services.iter()
            .filter(|service| service.rtype == "temperature")
            .map(|service| { service.rid.clone() })
            .collect::<Vec<String>>();
        temperature_devices.append(&mut iter);
    }

    for device in temperature_devices {
        let response = client.send(hue::requests::TemperatureRequest { id: device })?;
        println!("Response: {:?}", response.data);
        if let Some(data) = response.data.get(0) {
            let metric = Temperature {
                temperature: data.temperature.temperature.as_f64().unwrap(),
                device: device_names.get(&data.owner.rid).unwrap().to_string(),
                device_id: data.owner.rid.clone(),
            };

            let res = telegrafClient.write(&metric);
            if let Err(e) = res {
                println!("Failed to write telegraf metric: {:?}", e);
            }
        }
    }

    Ok(())
}
