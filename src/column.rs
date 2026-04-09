use core::panic;
use std::fmt::{Display, format, write};


#[derive(Debug)]
pub struct Column<T>{
    data: Vec<T>
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
    fn filter_mask(&mut self, statement: fn(T) -> bool) -> Vec<bool>{
        self.data.iter().map(|v| statement(v.clone())).collect()
    }
    fn apply_mask(&mut self, mask: Vec<bool>) -> Column<T>{
        let filtered = self.data
            .iter()
            .zip(mask)
            .filter_map(|(v, m)| if m == true {Some(v.clone())} else {None})
            .collect();
        Column { data: filtered }
        }
    pub fn filter_column(&mut self, statement: fn(T) -> bool) -> Column<T>{
        let mask = self.filter_mask(statement);
        self.apply_mask(mask)
    }

    pub fn map_column<U, F>(&self, statement: F) -> Column<U>
    where 
    F: Fn(&T) -> U{
        let data = self.data.iter().map(|v| statement(v)).collect();

        Column { data }
    }
    pub fn concat_columns<U>(&self, data: Vec<U>) -> Column<String>
        where 
        T: Display,
        U: Display{
        if self.data.len() != data.len(){
            panic!("Vectors must be same len");
        }
        let result: Vec<String> = self.data.iter().zip(data.iter())
        .map(|(a,b)| format!("{} {}", a,b)).collect();

        Column { data: result }
    }
}
