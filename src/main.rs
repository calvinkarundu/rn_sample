extern crate clap;
extern crate reqwest;
extern crate serde;

use std::collections::HashMap;
use clap::{Arg, App, SubCommand};

pub mod temperature;
pub mod currency;

trait Converter {
    fn convert(&self, &f64, &str, &str) -> f64;
}

struct TemperatureConverter;

impl Converter for TemperatureConverter {
    fn convert(&self, value: &f64, from: &str, to: &str) -> f64 {
        let result = match (from, to) {
            ("c","f") => (value * 9.0/5.0) + 32.0,
            ("f","c") => (value - 32.0) * 5.0/9.0,
            _ => 0f64,
        };

        result
    }
}

fn main () {
    let mut look_up: HashMap<&str, Box<Converter>> = HashMap::new();
    look_up.insert("temperature", Box::new(TemperatureConverter));

    let matches = App::new("sample")
        .version("1.0.0")
        .author("rustacean")
        .subcommand(
            SubCommand::with_name("temperature")
                .about("Convert temperature")
                .arg(Arg::with_name("value")
                    .short("v")
                    .long("value")
                    .takes_value(true))
                .arg(Arg::with_name("units")
                    .short("u")
                    .long("units")
                    .takes_value(true))
        )
        .subcommand(
            SubCommand::with_name("currency")
                .about("Convert currency")
                .arg(Arg::with_name("value")
                    .short("v")
                    .long("value")
                    .takes_value(true))
                .arg(Arg::with_name("units")
                    .short("u")
                    .long("units")
                    .takes_value(true))
        )
        .get_matches();

    match matches.subcommand() {
        ("currency", Some(sub_m)) => {
            let value: &str = sub_m.value_of("value").unwrap();
            let units: &str = sub_m.value_of("units").unwrap();
            let res = currency::convert(value, units);
            println!("{}", res);
        },

        ("temperature", Some(sub_m)) => {
            let value: &str = sub_m.value_of("value").unwrap();
            let units: &str = sub_m.value_of("units").unwrap();
            let res = temperature::convert(value, units);
            let converter = look_up.get("temperature").unwrap();
            let res = converter.convert(&67f64, "c", "f");
            println!("{}", res);
        }
        _ => println!("Unsupported conversion")
    }
}

/*
    Simple version without CLAP
*/

// fn main() {
//     let mut action_string = String::new();

//     io::stdin()
//         .read_line(&mut action_string)
//         .expect("Failed to read line");

//     let numbers: Vec<&str> = action_string.split("+").collect();

//     let num1 = numbers[0];
//     let num2 = numbers[1];

//     let num1: u32 = match num1.trim().parse() {
//         Ok(num) => num,
//         Err(err) => {
//             eprintln!("Error: {:?}", err);
//             std::process::exit(1);
//         }
//     };

//     let num2: u32 = match num2.trim().parse() {
//         Ok(num) => num,
//         Err(err) => {
//             eprintln!("Error: {:?}", err);
//             std::process::exit(1);
//         }
//     };

//     let result = num1 + num2;

//     println!("Result is {}", result)
// }
