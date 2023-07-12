use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum ValueType {
    STRING(String),
    LIST(HashMap<String, ValueType>),
}

impl PartialEq for ValueType {
    fn eq(&self, other: &Self) -> bool {
        match self {
            ValueType::STRING(str1) => {
                if let ValueType::STRING(str2) = other {
                    str2 == str1
                } else {
                    false
                }
            }
            ValueType::LIST(map1) => {
                if let ValueType::LIST(map2) = other {
                    map1 == map2
                } else {
                    false
                }
            }
        }
    }
}

impl ValueType {
    pub fn get_str(&self, path: &str) -> Result<String> {
        if path == "" {
            if let ValueType::STRING(val) = self {
                return Ok(val.clone().to_owned());
            } else {
                return Err(anyhow::anyhow!("error: query doesn't match the graph structure"))
            }
        }
        let parts = path.split_once('.').unwrap_or((path, ""));

        if let ValueType::LIST(map) = self {
            let value = map.get(parts.0).ok_or(anyhow::anyhow!("error"))?;
            return value.get_str(parts.1)
        } else {
            return Err(anyhow::anyhow!("error: query doesn't match the graph structure"))
        }
    }

    ///always returns the list type but it is kept wrapped in `ValueType` so you can still use the .get_str and .get_node methods
    pub fn get_node(&self, path: &str) -> Result<ValueType> {
        if path == "" {
            if let ValueType::LIST(_) = self {
                return Ok(self.clone());
            } else {
                return Err(anyhow::anyhow!("error: query doesn't match the graph structure"));
            }
        }
        let parts = path.split_once('.').unwrap_or((path, ""));

        if let ValueType::LIST(map) = self {
            let value = map.get(parts.0).ok_or(anyhow::anyhow!("error"))?;
            return value.get_node(parts.1)
        } else {
            return Err(anyhow::anyhow!("error: query doesn't match the graph structure"))
        }
    }
}

pub fn from_packet(data: Vec<u8>) -> Result<ValueType> {
    let data = String::from_utf8(data)?;
    let data: Vec<&str> = data.split(' ').collect();
    let (_, map) = parse_list(&data.get(1..).unwrap())?;

    Ok(ValueType::LIST(map))
}

fn parse_list(data: &[&str]) -> Result<(usize, HashMap<String, ValueType>)> {
    let mut map = HashMap::new();
    let mut index = 0;
    while index < data.len() {
        let key = *data.get(index).unwrap();
        if key == "]" {
            return Ok((index, map));
        }
        index += 1;
        let value = *data.get(index).unwrap();
        if value == "[" {
            let (i, val) = parse_list(data.get(index + 1..).unwrap())?;
            map.insert(key.to_owned(), ValueType::LIST(val));
            index += 1 + i;
        } else {
            map.insert(key.to_owned(), ValueType::STRING(value.to_owned()));
        }
        index += 1;
    }

    Ok((index, map))
}

pub fn to_packet(data: ValueType) -> Vec<u8> {
    if let ValueType::LIST(map) = data {
        let string = write_list(map);
        string.into_bytes()
    } else {
        panic!("Root ValueType must be a LIST variant")
    }
}

fn write_list(map: HashMap<String, ValueType>) -> String {
    let mut string = "[".to_owned();
    for element in map {
        string.push(' ');
        string.push_str(&element.0);
        string.push(' ');
        match element.1 {
            ValueType::STRING(value) => {
                string.push_str(&value);
            }
            ValueType::LIST(map) => string.push_str(&write_list(map)),
        }
    }
    string.push_str(" ]");
    string
}
