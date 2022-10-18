use crate::arg::Arg;
use crate::comparison_operator::ComparisonOperator;
use crate::value::Value;

#[derive(PartialEq)]
pub enum Instruction {
    Var(String, Value),
    Add(String, Value),
    Print(Value),
    Input(Value),
    Func(String, Vec<String>),
    FuncEnd,
    Return(Value),
    Call(String, String, Vec<Value>),
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
    While(Value, ComparisonOperator, Value),
    WhileEnd,
    Break,
    Import(String)
}

impl Instruction {
    pub fn parse(instruction: &str, args: Vec<Arg>) -> Instruction {
        match instruction {
            "VAR" => Instruction::Var(args[0].get_name(), args[1].get_value()),
            "ADD" => Instruction::Add(args[0].get_name(), args[1].get_value()),
            "PRINT" => Instruction::Print(args[0].get_value()),
            "FUNC" => {
                let mut func_args = vec!();

                for i in 1..args.len() {
                    func_args.push((args[i].get_name()));
                }

                Instruction::Func(args[0].get_name(), func_args)
            },
            "FUNC_END" => Instruction::FuncEnd,
            "RETURN" => Instruction::Return(args[0].get_value()),
            "CALL" => {
                let mut func_args = vec!();

                for i in 2..args.len() {
                    func_args.push(args[i].get_value());
                }

                Instruction::Call(args[0].get_name(), args[1].get_name(), func_args)
            },
            "ARR_VAR" => {
                let mut array = vec!();

                for i in 1..args.len() {
                    array.push(args[i].get_value());
                }

                Instruction::ArrVar(args[0].get_name(), array)
            },
            "WHILE" => Instruction::While(args[0].get_value(), args[1].get_op(), args[2].get_value()),
            "WHILE_END" => Instruction::WhileEnd,
            "BREAK" => Instruction::Break,
            _ => panic!("Invalid instruction {instruction}")
        }
    }
}
