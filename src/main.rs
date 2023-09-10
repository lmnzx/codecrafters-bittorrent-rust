use serde_bencode::{from_str, value::Value};
use serde_json;
use std::env;

fn format(v: &Value) -> String {
    return match v {
        Value::Bytes(b) => String::from_utf8(b.clone()).unwrap(),
        Value::Int(i) => format!("{}", i.to_string()),
        Value::List(l) => format!(
            "[{}]",
            l.iter()
                .map(|v| format(v))
                .collect::<Vec<String>>()
                .join(", ")
        ),
        _ => panic!("unsupported type"),
    };
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        let encoded_value = &args[2];
        let decoded_value = from_str::<Value>(encoded_value).unwrap();
        println!("{}", format(&decoded_value));
    } else {
        println!("unknown command: {}", args[1])
    }
}
