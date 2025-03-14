use telegraf::Metric;

#[derive(Metric)]
#[measurement = "temperature"]
pub struct Temperature {
    pub temperature: f64,
    #[telegraf(tag)]
    pub device: String,
    #[telegraf(tag)]
    pub device_id: String,
}
