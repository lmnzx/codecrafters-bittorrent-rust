use serde_bencode::value::Value as BencodeValue;
use serde_json::Value as JsonValue;

pub fn decode(value: &[u8]) -> JsonValue {
    if let Ok(v) = serde_bencode::from_bytes::<BencodeValue>(value) {
        bencode_to_json(&v)
    } else {
        panic!("Unhandled encoded value");
    }
}

pub fn bencode_to_json(v: &BencodeValue) -> JsonValue {
    match v {
        BencodeValue::Bytes(b) => {
            JsonValue::String(b.iter().map(|b| *b as char).collect::<String>())
        }
        BencodeValue::Int(i) => JsonValue::Number((*i).into()),
        BencodeValue::List(l) => JsonValue::Array(l.iter().map(bencode_to_json).collect()),
        BencodeValue::Dict(d) => JsonValue::Object(
            d.iter()
                .filter_map(|(k, v)| {
                    String::from_utf8(k.clone())
                        .ok()
                        .map(|ks| (ks, bencode_to_json(&v.clone())))
                })
                .collect(),
        ),
    }
}
