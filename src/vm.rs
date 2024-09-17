use std::collections::LinkedList;
//use crate::object::Object;
use std::rc::Rc;
use std::cell::RefCell;
use crate::chunk::{Chunk, OpCode};

pub struct VirtualMachine {
    //objects: LinkedList<Rc<RefCell<dyn Object>>>,
    //stack: Vec<Rc<RefCell<dyn Object>>>
}

impl VirtualMachine {
    pub fn new() -> Self {
        Self {
            //objects: LinkedList::new(),
            //stack: Vec::new()
        }
    }

    pub fn interpret_chunk(&mut self, chunk: &mut Chunk) {
        
    }

}