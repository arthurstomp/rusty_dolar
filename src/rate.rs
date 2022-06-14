use serde::Deserialize;

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

#[derive(Deserialize, Debug)]
pub struct Result {
    success: bool,
    query: Query,
    info: Info,
    date: String,
    result: f32,
}
