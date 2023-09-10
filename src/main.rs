use serde_bencode::{from_str, value::Value};
use std::env;

/*
"{\"foo\":\"pineapple\",\"hello\":52}\n"
"{\"foo\": \"pineapple\",\"hello\": 52}\n"
*/

fn format(v: &Value) -> String {
    return match v {
        Value::Bytes(b) => format!("{:?}", String::from_utf8(b.clone()).unwrap()),
        Value::Int(i) => i.to_string(),
        Value::List(l) => format!(
            "[{}]",
            l.iter()
                .map(|v| format(v))
                .collect::<Vec<String>>()
                .join(", ")
        ),
        Value::Dict(d) => {
            let mut r = Vec::<String>::new();
            for (k, v) in d {
                let key = String::from_utf8_lossy(k).to_string();
                r.push(format!("\"{}\":{}", key, format(v)));
            }
            r.sort();
            format!("{{{}}}", r.join(","))
        }
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
