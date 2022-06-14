mod api;

use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = api::Client::new(env::var("FIXER_API_KEY")?);
    let rate = client.rate(String::from("USD"), String::from("BRL"), 1.0)?;

    println!("{:#?}", rate);

    Ok(())
}
