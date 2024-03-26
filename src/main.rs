use bittorrent::Bencode;
use std::env;

fn decode_bencoded_value(encoded_value: &str, cursor: &mut i64) -> Bencode {
    if Bencode::check_list(encoded_value) {
        let list = decode_bencoded_list(encoded_value, cursor);
        return list;
    }
    if Bencode::check_int(encoded_value) {
        let integer = decode_bencoded_integer(encoded_value, cursor);
        return integer;
    }
    if Bencode::check_string(encoded_value) {
        let string = decode_bencoded_string(encoded_value, cursor);
        return string;
    } else {
        panic!("Unhandled encoded value: {}", encoded_value)
    }
}

fn decode_bencoded_string(encoded_value: &str, cursor: &mut i64) -> Bencode {
    let colon_index = encoded_value.find(':').unwrap();
    let number_string = &encoded_value[..colon_index];
    let number = number_string.parse::<i64>().unwrap();
    let end_index = colon_index + 1 + number as usize;
    let string = &encoded_value[colon_index + 1..end_index];

    (*cursor) = end_index as i64;
    return Bencode::String(string.to_string());
}

fn decode_bencoded_integer(encoded_value: &str, cursor: &mut i64) -> Bencode {
    let end_index = encoded_value.find('e').unwrap();
    let integer_string = &encoded_value[1..end_index];
    let integer = integer_string.parse::<i64>().unwrap();

    (*cursor) = end_index as i64 + 1; // discard the last `e`
    return Bencode::Int(integer);
}

fn decode_bencoded_list(encoded_value: &str, cursor: &mut i64) -> Bencode {
    let end_index = encoded_value.rfind('e').unwrap();
    let mut list_string = &encoded_value[1..end_index];

    let mut values_list: Vec<Bencode> = Vec::new();
    while list_string.len() > 0 {
        // println!(">>> {} {} {:?}", cursor, list_string, values_list);
        let value = decode_bencoded_value(list_string, cursor);
        values_list.push(value);
        list_string = &list_string[*cursor as usize..list_string.len()];
    }

    return Bencode::List(values_list);
}

// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        let cursor: &mut i64 = &mut 0;
        let encoded_value = &args[2];
        let decoded_value = decode_bencoded_value(encoded_value, cursor);
        println!("{}", decoded_value.as_serde_json_value().to_string());
    } else {
        println!("unknown command: {}", args[1])
    }
}
