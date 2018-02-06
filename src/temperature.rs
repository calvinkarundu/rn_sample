use std;

pub fn convert (value: &str, units: &str) -> String {
    let value: f32 = match value.trim().parse() {
        Ok(num) => num,
        Err(err) => {
            eprintln!("Error: {:?}", err);
            std::process::exit(1);
        }
    };

    let result = match units {
        "c-f" => {
            let converted = (value * 9.0/5.0) + 32.0;
            format!("{}c to f is {}f", value, converted)
        },
        "f-c" => {
            let converted = (value - 32.0) * 5.0/9.0;
            format!("{}f to c is {}c", value, converted)
        },
        _ => format!("{}", "Unsupported units")
    };

    result
}
