use reqwest::blocking::Client;
use serde::Deserialize;
use std::env;

#[derive(Deserialize, Debug)]
struct RateResultQuery {
    from: String,
    to: String,
    amount: i32,
}

#[derive(Deserialize, Debug)]
struct RateResultInfo {
    timestamp: i32,
    rate: f32,
}

#[derive(Deserialize, Debug)]
struct RateResult {
    success: bool,
    query: RateResultQuery,
    info: RateResultInfo,
    date: String,
    result: f32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let req = client
        .get("https://api.apilayer.com/fixer/convert")
        .header("apiKey", env::var("FIXER_API_KEY")?)
        .query(&[("to", "BRL"), ("from", "USD"), ("amount", "1")]);

    let res_json: RateResult = req.send()?.json()?;
    println!("{:#?}", res_json);

    Ok(())
}
