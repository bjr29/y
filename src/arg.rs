use crate::comparison_operator::ComparisonOperator;
use crate::is_valid_var_name;
use crate::value::Value;
use crate::value_type::ValueType;

#[derive(Clone)]
pub enum Arg {
    Value(Value),
    ValueType(ValueType),
    ComparisonOperator(ComparisonOperator),
    Name(String),
}

impl Arg {
    pub fn parse(arg: &String) -> Self {
        match Value::parse(arg) {
            Ok(value) => {
                println!("Parsing arg '{arg}' as a value");
                return Arg::Value(value);
            }
            Err(_) => {}
        }

        match ValueType::parse(arg) {
            Ok(value) => {
                println!("Parsing arg '{arg}' as a value type");
                return Arg::ValueType(value);
            }
            Err(_) => {}
        }

        match ComparisonOperator::parse(arg) {
            Ok(value) => {
                println!("Parsing arg '{arg}' as an operator");
                return Arg::ComparisonOperator(value);
            }
            Err(_) => {}
        }

        if is_valid_var_name(arg) {
            println!("Parsing arg '{arg}' as a variable");
            return Arg::Name(arg.to_string());
        }

        panic!("Failed to parse argument")
    }

    pub fn get_name(&self) -> String {
        match self {
            Arg::Name(value) => value,
            _ => panic!("Failed to get name, actual value '{}'", self.get_type_name())
        }.to_string()
    }

    pub fn get_value(&self) -> Value {
        match self.clone() {
            Arg::Value(value) => value,
            Arg::Name(value) => Value::Name(value),
            _ => panic!("Failed to get value, actual value '{}'", self.get_type_name())
        }
    }

    pub fn get_type(&self) -> ValueType {
        match self.clone() {
            Arg::ValueType(value) => value,
            _ => panic!("Failed to get type, actual value '{}'", self.get_type_name())
        }
    }

    pub fn get_op(&self) -> ComparisonOperator {
        match self.clone() {
            Arg::ComparisonOperator(value) => value,
            _ => panic!("Failed to get operator', actual value '{}'", self.get_type_name())
        }
    }

    fn get_type_name(&self) -> &str {
        match self.clone() {
            Arg::Value(_) => { "value" }
            Arg::ValueType(_) => { "type" }
            Arg::ComparisonOperator(_) => { "operator" }
            Arg::Name(_) => { "name" }
        }
    }
}
