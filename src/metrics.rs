use telegraf::Metric;

#[derive(Metric)]
#[measurement = "temperature"]
pub struct TemperatureMetric {
    pub temperature: f64,
    #[telegraf(tag)]
    pub device: String,
    #[telegraf(tag)]
    pub device_id: String,
}

#[derive(Metric)]
#[measurement = "light_level"]
pub struct LightLevelMetric {
    pub light_level: i64,
    #[telegraf(tag)]
    pub device: String,
    #[telegraf(tag)]
    pub device_id: String,
}
