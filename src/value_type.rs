#[derive(Clone, PartialEq)]
pub enum ValueType {
    Byte,
    Int,
    Float,
    Bool,
    Array(Box<ValueType>),
    Void
}

impl ValueType {
    pub fn parse(arg: &String) -> Result<Self, ()> {
        let result = match arg.as_str() {
            "byte" => Ok(ValueType::Byte),
            "int" => Ok(ValueType::Int),
            "float" => Ok(ValueType::Float),
            "bool" => Ok(ValueType::Bool),
            "void" => Ok(ValueType::Void),

            _ => Err(())
        };

        match result {
            Ok(value) => {
                if value == ValueType::Void {
                    return Err(());
                }

                Ok(ValueType::Array(Box::new(value)))
            }
            Err(_) => Err(())
        }
    }
}
