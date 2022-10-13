use std::fmt;
use std::fmt::{Display, Formatter};
use crate::{get_app_data, get_var_value, is_valid_var_name};

#[derive(Clone)]
pub enum Value {
    Name(String),
    Byte(u8),
    Int(i32),
    Float(f32),
    Bool(bool),
    Array(Vec<Value>),
    Null
}

impl Value {
    pub fn parse(string: &String) -> Result<Self, ()> {
        match string.as_str() {
            _ if string.starts_with("#") => {
                let mut cut_string = string.clone();
                cut_string.remove(0);
                
                Ok(Value::Byte(cut_string.parse::<u8>().unwrap()))
            },
            
            _ if string.starts_with("%") => {
                let mut cut_string = string.clone();
                cut_string.remove(0);

                Ok(Value::Float(cut_string.parse::<f32>().unwrap()))
            },
            
            _ if string == "true" || string == "false" => Ok(Value::Bool(string.parse::<bool>().unwrap())),
            
            "!" => Ok(Value::Null),

            _ => {
                match string.parse::<i32>() {
                    Ok(value) => { return Ok(Value::Int(value)); },
                    Err(_) => {}
                }

                Err(())
            }
        }
    }

    pub fn as_byte(&self) -> u8 {
        match self {
            Value::Byte(value) => { *value }
            Value::Name(var) => { get_var_value(var).as_byte() }
            _ => panic!("Value is not a byte, actual value is a {self}")
        }
    }

    pub fn as_int(&self) -> i32 {
        match self {
            Value::Int(value) => { *value }
            Value::Name(var) => { get_var_value(var).as_int() }
            _ => panic!("Value is not an int, actual value is a {self}")
        }
    }

    pub fn as_float(&self) -> f32 {
        match self {
            Value::Float(value) => { *value }
            Value::Name(var) => { get_var_value(var).as_float() }
            _ => panic!("Value is not a float, actual value is a {self}")
        }
    }

    pub fn as_array(&self) -> Vec<Value> {
        match self {
            Value::Array(value) => { value.clone() }
            Value::Name(var) => { get_var_value(var).as_array() }
            _ => panic!("Value is not an array, actual value is a {self}")
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut result = vec!();

        for value in self.as_array() {
            result.push(value.as_byte());
        }

        result
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self.clone() {
            Value::Name(value) => { write!(f, "name {value}") }
            Value::Byte(value) => { write!(f, "byte {value}") }
            Value::Int(value) => { write!(f, "int {value}") }
            Value::Float(value) => { write!(f, "float {value}") }
            Value::Bool(value) => { write!(f, "bool {value}") }
            Value::Array(value) => { write!(f, "array") }
            Value::Null => { write!(f, "null") }
        }
    }
}
