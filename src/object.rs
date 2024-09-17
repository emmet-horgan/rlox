use std::fmt::Display;
use std::collections::HashMap;
use crate::chunk::Chunk;

#[derive(Debug, Clone, Copy)]
pub enum Object {
    Struct,
    Function,
    Method,
    Instance,
    String,
    Number
}

pub struct SiennaStruct {
    name: String,
    methods: HashMap<String, SiennaFunction>
}

pub type NativeFunction = fn()

pub struct SiennaFunction {
    name: String,
    chunk: Chunk
}

pub struct SiennaString(String);

impl Display for SiennaString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct SiennaNumber(f64);

impl Display for SiennaNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_dyn_dispatch() {
        
    }
}