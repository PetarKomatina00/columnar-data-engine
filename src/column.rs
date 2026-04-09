use core::panic;
use std::fmt::{Display, format, write};


#[derive(Debug)]
pub struct Column<T>{
    pub data: Vec<T>
}
impl<T: Clone> Column<T>{
    pub fn new(data: Vec<T>) -> Self{
        Self {data }
    }
    pub fn push(&mut self, value: T){
        &self.data.push(value);
    }
    pub fn len(&mut self){
        &self.data.len();
    }
    pub fn is_empty(&mut self) -> bool{
        self.data.is_empty()
    }
}
