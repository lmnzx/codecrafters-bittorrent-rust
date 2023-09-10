use serde_bencode::{self, value::Value};
use std::env;

fn decode(encoded_value: &str) -> Value {
    return serde_bencode::from_str::<Value>(encoded_value).unwrap();
}

fn format(value: &Value) -> String {
    return match value {
        Value::Bytes(bytes) => format!("{:?}", std::str::from_utf8(bytes).unwrap()),
        Value::Int(i) => i.to_string(),
        Value::List(list) => format!(
            "[{}]",
            list.iter().map(format).collect::<Vec<String>>().join(",")
        ),

        Value::Dict(dict) => {
            let mut result = Vec::<String>::new();
            for (key, value) in dict {
                let key_str = String::from_utf8_lossy(key).to_string();
                result.push(format!("\"{}\":{}", key_str, format(value)));
            }
            result.sort();
            format!("{{{}}}", result.join(","))
        }
    };
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];
    if command == "decode" {
        let encoded_value = &args[2];

        let decoded_value = decode(encoded_value);

        println!("{}", format(&decoded_value));
    } else {
        println!("unknown command: {}", args[1])
    }
}
