use anyhow::Result;
use std::collections::HashMap;
use std::fmt::Display;

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
        if path.is_empty() {
            return if let ValueType::STRING(val) = self {
                Ok(val.to_owned())
            } else {
                Err(anyhow::anyhow!(
                    "error: query doesn't match the graph structure"
                ))
            };
        }
        let parts = path.split_once('.').unwrap_or((path, ""));

        if let ValueType::LIST(map) = self {
            let value = map.get(parts.0).ok_or(anyhow::anyhow!("error"))?;
            value.get_str(parts.1)
        } else {
            Err(anyhow::anyhow!(
                "error: query doesn't match the graph structure"
            ))
        }
    }

    ///always returns the list type but it is kept wrapped in `ValueType` so you can still use the .get_str and .get_node methods
    pub fn get_node(&self, path: &str) -> Result<ValueType> {
        if path.is_empty() {
            return if let ValueType::LIST(_) = self {
                Ok(self.clone())
            } else {
                Err(anyhow::anyhow!(
                    "error: query doesn't match the graph structure"
                ))
            };
        }
        let parts = path.split_once('.').unwrap_or((path, ""));

        if let ValueType::LIST(map) = self {
            let value = map.get(parts.0).ok_or(anyhow::anyhow!("error"))?;
            value.get_node(parts.1)
        } else {
            Err(anyhow::anyhow!(
                "error: query doesn't match the graph structure"
            ))
        }
    }
}

pub fn from_byte_vec(data: Vec<u8>) -> Result<ValueType> {
    let data = String::from_utf8(data)?;
    let data: Vec<String> = split_string(data);
    let (_, map) = parse_list(data.get(1..).unwrap())?;

    Ok(ValueType::LIST(map))
}

fn split_string(mut val: String) -> Vec<String> {
    let mut parts: Vec<String> = Vec::new();
    while !val.is_empty() {
        let (part, rest) = val.split_once(' ').unwrap_or((val.as_str(), ""));
        if part == "[" || part == "]" {
            parts.push(part.to_owned());
            val = rest.to_owned();
            continue;
        }
        let str_len = part.parse::<usize>().unwrap();
        let (part, rest) = rest.split_at(str_len);
        parts.push(part.to_owned());
        val = rest.trim().to_owned();
    }

    parts
}

fn parse_list(data: &[String]) -> Result<(usize, HashMap<String, ValueType>)> {
    let mut map = HashMap::new();
    let mut index = 0;
    while index < data.len() {
        let key = data.get(index).unwrap().clone();
        if key == "]" {
            return Ok((index, map));
        }
        index += 1;
        let value = data.get(index).unwrap().clone();
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

pub fn to_byte_vec(data: ValueType) -> Vec<u8> {
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
        string.push_str(&format!(" {} {} ", element.0.len(), element.0));
        match element.1 {
            ValueType::STRING(value) => {
                string.push_str(&format!("{} {}", value.len(), value));
            }
            ValueType::LIST(map) => string.push_str(&write_list(map)),
        }
    }
    string.push_str(" ]");
    string
}

fn write_tree(node: ValueType, f: &mut std::fmt::Formatter<'_>, depth: usize) -> std::fmt::Result {
    match node {
        ValueType::STRING(val) => writeln!(f, "{}", val),
        ValueType::LIST(map) => {
            writeln!(f, "[")?;
            for (key, value) in map {
                for _ in 0..depth {
                    write!(f, "  ")?;
                }
                write!(f, "  {} ", key)?;
                write_tree(value, f, depth + 1)?;
            }
            for _ in 0..depth {
                write!(f, "  ")?;
            }
            writeln!(f, "]")
        }
    }
}

impl Display for ValueType {
    //make this nicer
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write_tree(self.clone(), f, 0)
    }
}
