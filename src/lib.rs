use serde_json;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Bencode {
    String(String),
    Int(i64),
    List(Vec<Bencode>),
    Dict(),
}

#[allow(dead_code)]
impl Bencode {
    pub fn check_list(encoded_value: &str) -> bool {
        return encoded_value.chars().next().unwrap() == 'l';
    }

    pub fn check_string(encoded_value: &str) -> bool {
        return encoded_value.chars().next().unwrap().is_digit(10);
    }

    pub fn check_int(encoded_value: &str) -> bool {
        return encoded_value.chars().next().unwrap() == 'i';
    }

    pub fn check_dict(_encoded_value: &str) -> bool {
        todo!()
    }

    pub const fn is_string(&self) -> bool {
        matches!(*self, Bencode::String(_))
    }

    pub const fn is_int(&self) -> bool {
        matches!(*self, Bencode::Int(_))
    }

    pub const fn is_list(&self) -> bool {
        matches!(*self, Bencode::List(_))
    }

    pub const fn is_dict(&self) -> bool {
        matches!(*self, Bencode::Dict())
    }

    pub fn as_string(&self) -> Option<&String> {
        match self {
            Bencode::String(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_int(&self) -> Option<i64> {
        match *self {
            Bencode::Int(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_list(&self) -> Option<&Vec<Bencode>> {
        match self {
            Bencode::List(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_serde_json_value(&self) -> serde_json::Value {
        match self {
            Bencode::String(v) => serde_json::Value::String(v.clone()),
            Bencode::Int(v) => serde_json::Value::Number(serde_json::Number::from(*v)),
            Bencode::List(v) => {
                let values: Vec<serde_json::Value> =
                    v.iter().map(|b| b.as_serde_json_value()).collect();

                return serde_json::Value::Array(values);
            }
            Bencode::Dict() => todo!(),
        }
    }
}
