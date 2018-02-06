extern crate reqwest;
extern crate serde_json;

use std;

pub fn convert (value: &str, units: &str) -> String {
    let value: f32 = match value.trim().parse() {
        Ok(num) => num,
        Err(err) => {
            eprintln!("Error: {:?}", err);
            std::process::exit(1);
        }
    };

    let currency_units: Vec<&str> = units.split("-").collect();

    let from = currency_units[0].to_uppercase();
    let to = currency_units[1].to_uppercase();

    let rate = get_rate(&from, &to);
    let result = value * rate;

    format!("{}{}", result, to)
}

fn get_rate (from: &str, to: &str) -> f32 {
    let url = format!("https://api.fixer.io/latest?sym
bols={},{}", from, to);
    let mut resp = reqwest::get(&url).unwrap();

    if resp.status().as_u16() == 200 {
        let data: serde_json::Value = resp.json().unwrap();
        let rates: serde_json::Value = data.get("rates").unwrap().clone();
        let conversion: f64 = rates[to].as_f64().unwrap();

        conversion as f32
    } else {
        println!("Error: {:?}", "got non 200 status code");
        std::process::exit(1);
    }
}
