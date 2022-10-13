use std::env::args;
use std::fmt::Pointer;
use std::fs;
use std::sync::{Mutex, MutexGuard};
use lazy_static::lazy_static;
use instruction::Instruction;
use crate::app_data::AppData;
use crate::arg::Arg;
use crate::value::Value;

mod instruction;
mod value;
mod value_type;
mod comparison_operator;
mod arg;
mod app_data;

lazy_static! {
    static ref APP_DATA: Mutex<AppData> = Mutex::new(AppData::new());
}

fn main() {
    let filepath = args()
        .nth(1)
        .expect("Expected path included in arguments!");

    println!("Path: {}", filepath);

    let code = fs::read_to_string(filepath).unwrap();

    let instructions = parse_code(code);

    execute_code(&instructions);
}

fn parse_code(code: String) -> Vec<Instruction> {
    let mut instructions = vec!();

    for line in code.split(";") {
        let mut splits = line.split_whitespace();

        let instruction = match splits
            .nth(0) {
            None => { continue }
            Some(value) => { value }
        };

        let string_args = splits
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        let mut args = vec!();

        for i in 0..string_args.len() {
            args.push(Arg::parse(&string_args[i]));
        }

        instructions.push(Instruction::parse(instruction, args));
    }

    instructions
}

fn execute_code(instructions: &Vec<Instruction>) {
    let mut line_numbers = vec![0];
    let mut app = get_app_data();

    loop {
        let instruction = &instructions[line_numbers[line_numbers.len() - 1] as usize];

        match instruction {
            Instruction::Var(name, value) => {
                app.values.insert(name.clone(), value.clone());
            }
            Instruction::Add(name, value) => {
                let result = match value {
                    Value::Byte(x) => { Value::Byte(x + value.as_byte()) }
                    Value::Int(x) => { Value::Int(x + value.as_int()) }
                    Value::Float(x) => { Value::Float(x + value.as_float()) }

                    _ => panic!("Incompatible types to perform ADD operation")
                };

                app.values.insert(name.clone(), result);
            }
            Instruction::Print(value) => {
                let bytes = value.as_bytes();
                let string = String::from_utf8_lossy(&bytes);

                println!("{}", string);
            },
            Instruction::PrintChar(value) => {
                let byte = [value.as_byte()];

                print!("PRINT {}", String::from_utf8_lossy(&byte));
            },
            Instruction::Input(name) => {}
            Instruction::Func(name, value_type, args) => {}
            Instruction::FuncEnd => {}
            Instruction::Return(value) => {}
            Instruction::Call(name, args) => {}
            Instruction::If(a, comparison_operator, b) => {}
            Instruction::ElseIf(a, comparison_operator, b) => {}
            Instruction::Else => {}
            Instruction::EndIf => {}
            Instruction::ArrAppend(name, value, i) => {}
            Instruction::ArrRemove(name, i) => {}
            Instruction::ArrGet(name, i, _) => {}
            Instruction::FileRead(path, name) => {}
            Instruction::FileWrite(path, name, value) => {}
        };

        if line_numbers[0] >= instructions.len() - 1 {
            return;
        }

        let length = line_numbers.len();
        line_numbers[length - 1] += 1;

        println!("{}", line_numbers[length - 1]);
    }
}

fn get_app_data() -> MutexGuard<'static, AppData> {
    APP_DATA.lock().unwrap()
}

fn is_valid_var_name(string: &String) -> bool {
    let chars = string.chars();

    chars.clone().all(|c| c.is_alphanumeric() || c == '_')
        && chars.clone().nth(0).unwrap().is_alphabetic()
}

fn get_var_value(var: &String) -> Value {
    get_app_data().values.get(&*var).unwrap().clone()
}
