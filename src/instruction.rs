use crate::arg::Arg;
use crate::comparison_operator::ComparisonOperator;
use crate::value::Value;
use crate::value_type::ValueType;

pub enum Instruction {
    Var(String, Value),
    Add(String, Value),
    Print(Value),
    PrintChar(Value),
    Input(Value),
    Func(String, ValueType, Vec<(ValueType, String, Value)>),
    FuncEnd,
    Return(Value),
    Call(String, Vec<Value>),
    If(Value, ComparisonOperator, Value),
    ElseIf(Value, ComparisonOperator, Value),
    Else,
    EndIf,
    ArrVar(String, Vec<Value>),
    ArrAppend(String, Value, Value),
    ArrRemove(String, Value),
    ArrGet(String, Value, String),
    FileRead(String, String),
    FileWrite(String, String, Value),
}

impl Instruction {
    pub fn parse(instruction: &str, args: Vec<Arg>) -> Instruction {
        println!("Parsing: {instruction}");

        match instruction {
            "VAR" => Instruction::Var(args[0].get_name(), args[1].get_value()),
            "ADD" => Instruction::Add(args[0].get_name(), args[1].get_value()),
            "PRINT" => Instruction::Print(args[0].get_value()),
            "PRINT_CHAR" => Instruction::PrintChar(args[0].get_value()),
            "FUNC" => {
                let mut func_args = vec!();

                for i in (2..args.len()).step_by(3) {
                    func_args.push((args[i].get_type(), args[i + 1].get_name(), args[i + 2].get_value()));
                }

                Instruction::Func(args[0].get_name(), args[1].get_type(), func_args)
            },
            "FUNC_END" => Instruction::FuncEnd,
            "RETURN" => Instruction::Return(args[0].get_value()),
            "CALL" => {
                let mut func_args = vec!();

                for i in 2..args.len() {
                    func_args.push(args[i].get_value());
                }

                Instruction::Call(args[0].get_name(), func_args)
            },
            "ARR_VAR" => {
                let mut array = vec!();

                for i in 1..args.len() {
                    array.push(args[i].get_value());
                }

                Instruction::ArrVar(args[0].get_name(), array)
            },
            _ => panic!("Invalid instruction {instruction}")
        }
    }
}
