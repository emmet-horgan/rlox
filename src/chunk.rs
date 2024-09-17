//use crate::object::Object;
use std::rc::Rc;
use std::cell::RefCell;

pub enum OpCode {
    Constant,
    Nil,
    True,
    False,
    Pop,
    GetLocal,
    SetLocal,
    GetGlobal,
    DefineGlobal,
    SetGlobal,
    GetProperty,
    SetProperty,
    Equal,
    Greater,
    Less,
    Add,
    Subtract,
    Multiply,
    Divide,
    Not,
    Negate,
    Print,
    Jump,
    JumpIfFalse,
    Loop,
    Call,
    Invoke,
    Closure,
    Return,
    Class,
    Method
}

pub struct Chunk {
    code: Vec<OpCode>,
    lines: Vec<u32>,
    //constants: Vec<Rc<RefCell<dyn Object>>>
}

