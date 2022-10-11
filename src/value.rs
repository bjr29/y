use crate::{get_app_data, is_valid_var_name};

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
            _ => panic!("Value is not a byte!")
        }
    }

    pub fn as_int(&self) -> i32 {
        match self {
            Value::Int(value) => { *value }
            _ => panic!("Value is not an int!")
        }
    }

    pub fn as_float(&self) -> f32 {
        match self {
            Value::Float(value) => { *value }
            _ => panic!("Value is not a float!")
        }
    }

    pub fn as_array(&self) -> Vec<Value> {
        match self {
            Value::Array(value) => { value.clone() }
            _ => panic!("Value is not an array!")
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
