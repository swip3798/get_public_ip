extern crate reqwest;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct IpAnswer {
    origin: String
}

fn get_as_text(url: String) -> Result<(String), Box<std::error::Error>> {
    let body = reqwest::get(url.as_str())?
        .text()?;
    return Ok(body.clone());
}

fn print(text: String) {
    println!("{}", text);
}

fn main() -> Result<(), Box<std::error::Error>> {
    let resp = get_as_text("https://httpbin.org/ip".to_string());
    let mut answer = "{}".to_string();
    if let Ok(v) = resp {
        answer = v;
    }
    let deserialized: IpAnswer = serde_json::from_str(&answer).unwrap();
    let split = deserialized.origin.split(",");
    let vec: Vec<&str> = split.collect();
    print(vec[0].to_string());
    return Ok(());
}
