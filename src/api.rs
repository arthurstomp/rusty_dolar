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
    success: bool,
    query: Query,
    info: Info,
    date: String,
    result: f32,
}

#[derive(Deserialize, Debug)]
struct Query {
    from: String,
    to: String,
    amount: i32,
}

#[derive(Deserialize, Debug)]
struct Info {
    timestamp: i32,
    rate: f32,
}
