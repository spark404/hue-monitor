mod hue;
mod config;
mod metrics;

use crate::hue::client::{Client, ClientConfig};
use crate::hue::requests::GetById;
use clap::Parser;
use config::*;
use metrics::*;
use std::error::Error;
use std::path::PathBuf;
use std::process::exit;
use std::time::Instant;
use std::{fs, thread};
use telegraf::{Client as TelegrafClient, Metric};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(
        short,
        long,
        value_name = "FILE",
        default_value = "/etc/hue_monitor.toml"
    )]
    config: PathBuf,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
}


fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    println!("Starting hue monitor...");
    println!("Using configuration file at: {:?}", cli.config);

    let toml_content = fs::read_to_string(cli.config)?;
    let config = parse_config(toml_content)?;

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
    let device_resources = client.send(hue::requests::DevicesRequest {})?;

    let mut temperature_resources: Vec<String> = vec!();
    for device_resource in &device_resources.data {
        let mut iter = device_resource.services.iter()
            .filter(|service| service.rtype == "temperature")
            .map(|service| { service.rid.clone() })
            .collect::<Vec<String>>();
        temperature_resources.append(&mut iter);
    }

    for device in temperature_resources {
        let response = client.send(hue::requests::TemperatureRequest { id: device })?;
        if let Some(temp_device) = response.data.first() {
            let device = device_resources.get_by_id(&temp_device.owner.rid);
            let metric = Temperature {
                temperature: temp_device.temperature.temperature.as_f64().unwrap(),
                device: device.unwrap().metadata.name.clone(),
                device_id: device.unwrap().id.clone(),
            };

            let res = telegraf_client.write(&metric);
            if let Err(e) = res {
                eprintln!("Failed to write telegraf metric: {:?}", e);
            }
        }
    }

    Ok(())
}

