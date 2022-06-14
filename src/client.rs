use crate::rate;
use reqwest::blocking::Client as C;

pub struct Client {
    key: String,
}

impl Client {
    pub fn new(key: String) -> Client {
        Client { key }
    }

    pub fn rate(&self, from: String, to: String, amount: f32) -> reqwest::Result<rate::Result> {
        let res = C::new()
            .get("https://api.apilayer.com/fixer/convert")
            .header("apiKey", &self.key)
            .query(&[("from", from), ("to", to), ("amount", amount.to_string())])
            .send()?;

        res.json::<rate::Result>()
    }
}
