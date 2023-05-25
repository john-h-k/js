use anyhow::anyhow;
use slotmap::{new_key_type, SlotMap};
use std::{
    collections::{HashMap, VecDeque},
    io::{self, Write},
};

use tap::prelude::*;

use log::trace;

pub struct Emitter {}

new_key_type! {
    pub struct StringKey;
}

#[derive(Debug)]
pub enum Opcode {
    Nop,
    Ret,
    Pop,
    PushStr(String),
    PushInt(i64),
    Call { target: String, num_args: usize },
}

#[derive(Debug)]
pub struct FuncDef {
    pub name: StringKey,
    pub args: Vec<()>,
    //pub ret_type: Ty,
    pub body: Vec<Opcode>,
}

pub struct ArgDef {
    name: StringKey,
    ty: Ty,
}

enum Ty {
    Unit,
    String,
    Integer,
    Real,
    Boolean,
}

#[derive(Debug)]
pub struct Interpreter {
    strings: SlotMap<StringKey, String>,
    functions: HashMap<StringKey, FuncDef>,
    main_key: Option<StringKey>,
}

pub type Result<T> = anyhow::Result<T>;

enum RtValue {
    String(String),
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            strings: SlotMap::with_key(),
            functions: HashMap::new(),
            main_key: None,
        }
    }

    pub fn register_literal(&mut self, s: String) -> StringKey {
        if s == "main" {
            self.strings.insert(s).tap(|s| self.main_key = Some(*s))
        } else {
            self.strings.insert(s)
        }
    }

    pub fn add_function(&mut self, func: FuncDef) {
        self.functions.insert(func.name, func);
    }

    pub fn execute(&mut self) -> Result<()> {
        let func = &self.functions[&self.main_key.unwrap()];

        let mut stack = VecDeque::new();

        for opcode in func.body.iter() {
            trace!("Executing op: {:?}", opcode);
            match opcode {
                Opcode::Nop => {}
                Opcode::Ret => break,
                Opcode::Pop => {
                    stack.pop_back();
                }
                Opcode::PushStr(str) => {
                    stack.push_back(RtValue::String(str.to_string()));
                }
                Opcode::PushInt(_) => todo!(),
                Opcode::Call { target, .. } => match target.as_str() {
                    "println" => Self::builtin_print(
                        stack
                            .pop_back()
                            .ok_or(anyhow!("Expected value on top of stack"))?,
                    ),
                    _ => todo!(),
                },
            }
        }

        Ok(())
    }

    fn builtin_print(value: RtValue) {
        let string = match value {
            RtValue::String(str) => str,
        };

        io::stdout()
            .lock()
            .write_all((string + "\n").as_bytes())
            .expect("Writing to `stdout` failed");
    }
}
