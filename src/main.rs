mod rate;

use reqwest::blocking::Client;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let req = client
        .get("https://api.apilayer.com/fixer/convert")
        .header("apiKey", env::var("FIXER_API_KEY")?)
        .query(&[("to", "BRL"), ("from", "USD"), ("amount", "1")]);

    let res_json: rate::Result = req.send()?.json()?;
    println!("{:#?}", res_json);

    Ok(())
}
