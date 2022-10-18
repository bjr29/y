use crate::Value;

#[derive(Clone, PartialEq, Debug)]
pub enum ComparisonOperator {
    Equals,
    NotEqual,
    Greater,
    GreaterOrEqual,
    Lesser,
    LesserOrEqual
}

impl ComparisonOperator {
    pub fn parse(arg: &String) -> Result<Self, ()> {
        match arg.as_str() {
            "=" => Ok(ComparisonOperator::Equals),
            "!=" => Ok(ComparisonOperator::NotEqual),
            ">" => Ok(ComparisonOperator::Greater),
            ">=" => Ok(ComparisonOperator::GreaterOrEqual),
            "<" => Ok(ComparisonOperator::Lesser),
            "<=" => Ok(ComparisonOperator::LesserOrEqual),
            _ => Err(())
        }
    }

    pub fn process(&self, a: &Value, b: &Value) -> bool {
        match self {
            ComparisonOperator::Equals => { a == b }
            ComparisonOperator::NotEqual => { a != b }
            ComparisonOperator::Greater => { a > b }
            ComparisonOperator::GreaterOrEqual => { a >= b }
            ComparisonOperator::Lesser => { a < b }
            ComparisonOperator::LesserOrEqual => { a <= b }
        }
    }
}
