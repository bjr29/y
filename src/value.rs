use std::fmt;
use std::fmt::{Display, Formatter};
use std::sync::MutexGuard;
use crate::{AppData, get_var_value};

#[derive(Clone, PartialEq, PartialOrd, Debug)]
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

    pub fn as_byte(&self, app: &MutexGuard<AppData>) -> Result<u8, ()> {
        match self {
            Value::Byte(value) => { Ok(*value) }
            Value::Name(var) => { Ok(get_var_value(var, app).as_byte(app)?) }
            _ => Err(())
        }
    }

    pub fn as_int(&self, app: &MutexGuard<AppData>) -> Result<i32, ()> {
        match self {
            Value::Int(value) => { Ok(*value) }
            Value::Name(var) => { Ok(get_var_value(var, app).as_int(app)?) }
            _ => Err(())
        }
    }

    pub fn as_float(&self, app: &MutexGuard<AppData>) -> Result<f32, ()> {
        match self {
            Value::Float(value) => { Ok(*value) }
            Value::Name(var) => { Ok(get_var_value(var, app).as_float(app)?) }
            _ => Err(())
        }
    }

    pub fn as_bool(&self, app: &MutexGuard<AppData>) -> Result<bool, ()> {
        match self {
            Value::Bool(value) => { Ok(*value) }
            Value::Name(var) => { Ok(get_var_value(var, app).as_bool(app)?) }
            _ => Err(())
        }
    }

    pub fn as_array(&self, app: &MutexGuard<AppData>) -> Result<Vec<Value>, ()> {
        match self {
            Value::Array(value) => { Ok(value.clone()) }
            Value::Name(var) => { Ok(get_var_value(var, app).as_array(app)?) }
            _ => Err(())
        }
    }

    pub fn as_bytes(&self, app: &MutexGuard<AppData>) -> Result<Vec<u8>, ()> {
        let mut result = vec!();

        let array = match self.as_array(app) {
            Ok(value) => { value }
            Err(_) => { return Err(()) }
        };

        for value in array {
            result.push(value.as_byte(app)?);
        }

        Ok(result)
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
