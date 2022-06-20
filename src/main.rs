mod api;

use serde::Serialize;
use std::env;

#[derive(Serialize, Debug)]
pub struct FinalResult {
    from: String,
    to: String,
    rate: f32,
    from_amount: i32,
    to_amount: f32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = api::Client::new(env::var("FIXER_API_KEY")?);
    let from_amount = 1.0;
    let result = client.rate(String::from("USD"), String::from("BRL"), from_amount)?;
    let final_result = FinalResult {
        from: result.query.from,
        to: result.query.to,
        rate: result.info.rate,
        from_amount: result.query.amount,
        to_amount: result.result,
    };
    let final_result_json = serde_json::to_string(&final_result)?;

    println!("{}", final_result_json);

    Ok(())
}
