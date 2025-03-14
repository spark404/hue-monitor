mod hue;

use crate::hue::client::{Client, ClientConfig};
use clap::Parser;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;
use std::process::exit;
use std::time::{Duration, Instant};
use std::{fs, thread};
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
    #[serde(default = "default_duration")]
    interval: Duration,
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
    println!("Using configuration file at: {:?}", cli.config);

    let toml_content = fs::read_to_string(cli.config)?;
    let config: Config = toml::from_str(&toml_content)?;

    let client = Client::new(ClientConfig {
        url: config.huebridge.url.clone(),
        token: config.huebridge.token.clone(),
        certificate: config.huebridge.certificate.clone(),
    });

    if let Err(e) = client {
        eprintln!("Error while creating telegraf client: {}", e);
        exit(1);
    }

    let client = client?;

    // test the connection
    let config_response = client.send(hue::requests::Config {})?;
    println!("Connected to Hue bridge: {} @ {}", config_response.name, config.huebridge.url);

    let telegraf_client = TelegrafClient::new(&config.telegraf.endpoint);
    if let Err(e) = telegraf_client {
        panic!("Failed to create Telegraf client: {:?}", e);
    }
    println!("Connected to Telegraf at: {}", config.telegraf.endpoint);

    println!("Interval set to {:?}", config.interval);

    let mut telegraf_client = telegraf_client.unwrap();

    let scheduler = thread::spawn(move || {
        let wait_time = config.interval;
        loop {
            let start = Instant::now();
            let runtime = start.elapsed();

            if let Err(e) = update(&client, &mut telegraf_client) {
                panic!("Failed to update process an update: {:?}", e);
            }

            if let Some(remaining) = wait_time.checked_sub(runtime) {
                thread::sleep(remaining);
            }
        }
    });

    scheduler.join().expect("Scheduler panicked");

    Ok(())
}

fn update(client: &Client, telegraf_client: &mut TelegrafClient) -> Result<(), Box<dyn Error>> {
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
        if let Some(data) = response.data.get(0) {
            let metric = Temperature {
                temperature: data.temperature.temperature.as_f64().unwrap(),
                device: device_names.get(&data.owner.rid).unwrap().to_string(),
                device_id: data.owner.rid.clone(),
            };

            let res = telegraf_client.write(&metric);
            if let Err(e) = res {
                eprintln!("Failed to write telegraf metric: {:?}", e);
            }
        }
    }
    Ok(())
}

fn default_duration() -> Duration {
    Duration::from_secs(5)
}