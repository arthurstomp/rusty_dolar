use serde::Deserialize;

pub struct Client {
    key: String,
}

impl Client {
    pub fn new(key: String) -> Self {
        Self { key }
    }

    pub fn rate(&self, from: String, to: String, amount: f32) -> reqwest::Result<Result> {
        reqwest::blocking::Client::new()
            .get("https://api.apilayer.com/fixer/convert")
            .header("apiKey", &self.key)
            .query(&[("from", from), ("to", to), ("amount", amount.to_string())])
            .send()?
            .json::<Result>()
    }
}

#[derive(Deserialize, Debug)]
pub struct Result {
    pub query: Query,
    pub info: Info,
    pub result: f32,
    success: bool,
    date: String,
}

#[derive(Deserialize, Debug)]
pub struct Query {
    pub from: String,
    pub to: String,
    pub amount: i32,
}

#[derive(Deserialize, Debug)]
pub struct Info {
    pub rate: f32,
    timestamp: i32,
}
