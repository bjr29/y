#[derive(Clone)]
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
}
