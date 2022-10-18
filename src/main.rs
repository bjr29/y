use std::collections::HashMap;
use std::env::args;
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

    save_functions(&instructions);

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

fn save_functions(instructions: &Vec<Instruction>) {
    let mut app = get_app_data();
    let mut previous = ("".to_string(), 0, vec![]);

    for i in 0..instructions.len() {
        let instruction = &instructions[i];

        match instruction {
            Instruction::Func(name, args) => {
                previous = (name.to_string(), i + 1, args.clone());
            }
            Instruction::FuncEnd => {
                app.functions.insert(previous.clone().0, (previous.clone().1, i + 1, previous.clone().2));
            }
            _ => {}
        };
    }
}

fn execute_code(instructions: &Vec<Instruction>) {
    let mut line_numbers = vec![0];
    let mut returns = vec![];
    let mut app = get_app_data();

    loop {
        let instruction = &instructions[*line_numbers.last().unwrap()];

        println!("Line numbers: {line_numbers:?}");

        match instruction {
            Instruction::Var(name, value) => {
                create_var(&mut app, name.to_string(), value.clone());
            }
            Instruction::Add(name, value) => {
                let result = match value {
                    Value::Byte(x) => { Value::Byte(x + value.as_byte(&app).unwrap()) }
                    Value::Int(x) => { Value::Int(x + value.as_int(&app).unwrap()) }
                    Value::Float(x) => { Value::Float(x + value.as_float(&app).unwrap()) }

                    _ => panic!("Incompatible types ({} & {}) to perform ADD operation", get_var_value(name, &app), value)
                };

                create_var(&mut app, name.to_string(), result);
            }
            Instruction::Print(value) => {
                if let Ok(x) = value.as_bool(&app) {
                    println!("{x}");

                } else if let Ok(x) = value.as_float(&app) {
                    println!("{x}");

                } else if let Ok(x) = value.as_int(&app) {
                    println!("{x}");

                } else if let Ok(x) = value.as_byte(&app) {
                    let byte = [x];
                    let result = String::from_utf8_lossy(&byte);

                    println!("{result}");

                } else if let Ok(x) = value.as_array(&app) {
                    println!("{:?}", x);

                } else if let Ok(x) = value.as_bytes(&app) {
                    let result = String::from_utf8_lossy(&x);

                    println!("{result}");

                } else {
                    println!("null");
                }
            }
            Instruction::Input(name) => {}
            Instruction::Func(name, _) => {
                jump_to_instruction(instructions, &mut line_numbers, Instruction::FuncEnd);
            }
            Instruction::FuncEnd => {
                close_scope(&mut app, &mut line_numbers);
            }
            Instruction::Return(value) => {
                create_var(&mut app, returns.pop().unwrap(), value.clone());
            }
            Instruction::Call(name, var, args) => {
                let function = &app.functions.get(name).unwrap().clone();

                returns.push(var.to_string());

                open_scope(&mut app, &mut line_numbers, function.0);

                for i in 0..args.len() {
                    let arg = args[i].clone();
                    let arg_name = &function.2[i];

                    create_var(&mut app, arg_name.clone(), arg.clone());
                }

                println!("Vars: {:?}", app.values);
            }
            Instruction::If(a, comparison_operator, b) => {}
            Instruction::ElseIf(a, comparison_operator, b) => {}
            Instruction::Else => {}
            Instruction::EndIf => {}
            Instruction::ArrVar(name, array) => {
                create_var(&mut app, name.to_string(), Value::Array(array.clone()));
            }
            Instruction::ArrAppend(name, value, i) => {}
            Instruction::ArrRemove(name, i) => {}
            Instruction::ArrGet(name, i, _) => {}
            Instruction::FileRead(path, name) => {}
            Instruction::FileWrite(path, name, value) => {}
            Instruction::While(a, comparison_operator, b) => {
                if comparison_operator.process(a, b) {
                    line_numbers.push(line_numbers.last().unwrap() + 1);

                } else {
                    jump_to_instruction(instructions, &mut line_numbers, Instruction::WhileEnd);
                }
            }
            Instruction::WhileEnd => {
                close_scope(&mut app, &mut line_numbers);
            }
            Instruction::Break => {
                close_scope(&mut app, &mut line_numbers);

                jump_to_instruction(instructions, &mut line_numbers, Instruction::WhileEnd);

                todo!("Add error check");
            }
            Instruction::Import(_) => {}
        };

        if line_numbers[0] >= instructions.len() - 1 {
            return;
        }

        let length = line_numbers.len();
        line_numbers[length - 1] += 1;
    }
}

fn jump_to_instruction(instructions: &Vec<Instruction>, line_numbers: &mut Vec<usize>, jump_to: Instruction) {
    for i in *line_numbers.last().unwrap()..instructions.len() {
        if instructions[i] == jump_to {
            let index = line_numbers.len() - 1;
            line_numbers[index] = i;

            break;
        }
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

fn get_var_value(var: &String, app: &MutexGuard<AppData>) -> Value {
    for i in (0..app.values.len()).rev() {
        for scope in app.values.get(i) {
            let result = scope.get(&*var);

            if result.is_some() {
                return result.unwrap().clone();
            }
        }
    }

    panic!("Variable {var} doesn't exist");
}

fn open_scope(app: &mut MutexGuard<'_, AppData>, line_numbers: &mut Vec<usize>, line: usize) {
    let mut new_scope = vec![HashMap::new()];
    let mut new_line = vec![line];

    app.values.append(&mut new_scope);
    line_numbers.append(&mut new_line);
}

fn close_scope(app: &mut MutexGuard<'_, AppData>, line_numbers: &mut Vec<usize>) {
    app.values.pop();
    line_numbers.pop();
}

fn create_var(app: &mut MutexGuard<'_, AppData>, name: String, value: Value) {
    let length = app.values.len() - 1;
    let _ = app.values[length].insert(name, value);
}
