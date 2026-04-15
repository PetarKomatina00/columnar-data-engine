use core::panic;
use std::{fmt::{self, Display, format, write}, thread::current};

use crate::{engine::{binary_engine, unary_engine}, operations::{concat::ConcatOp, filter::FilterOp, map::MapOp}};



pub enum Operations<T>{
    Map(Box<dyn FnMut(T) -> T>),
    Filter(Box<dyn Fn(&T) -> bool>),
    Concat(Column<T>)
}
#[derive(Debug, PartialEq, Clone)]
pub struct Column<T>{
    pub data: Vec<T>
}

pub struct LazyColumn<T>{
    pub columns: Column<T>,
    pub operation_list: Vec<Operations<T>>,
    pub metadata: Vec<&'static str>
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

impl<T> LazyColumn<T>{
    pub fn new(col: Column<T>) -> Self{
        Self{
            columns : col,
            operation_list: Vec::new(),
            metadata : Vec::new()
        }
    }
    pub fn map<F>(mut self, f: F) -> Self
    where F: FnMut(T) -> T + 'static{
        self.operation_list.push(Operations::Map(Box::new(f)));
        self.metadata.push("Added Map Op");
        self
    }
    pub fn filter<F>(mut self, f: F) -> Self
    where F: Fn(&T) -> bool + 'static{
        self.operation_list.push(Operations::Filter(Box::new(f)));
        self.metadata.push("Added Filter Op");
        self
    }
    pub fn concat<F>(mut self, input: Column<T>) -> Self{
        self.operation_list.push(Operations::Concat(input));
        self.metadata.push("Added Concat Op");
        self
    }

    pub fn collect(self) -> Column<T>{
        let mut current_data = self.columns;
        for operation in self.operation_list{
            current_data = match operation{
                Operations::Map(mut f) => {
                    Column {
                        data: current_data.data.into_iter().map(|x| f(x)).collect()
                    }
                }
                Operations::Filter(f) => {
                    Column { data: current_data.data.into_iter().filter(|x| f(x)).collect() }
                }
                Operations::Concat(input) => {
                    let mut data = current_data.data;
                    data.extend(input.data);
                    Column { data: data}
                }
            }
        }
        current_data
    }
}
// impl<T: Display> Operations<T>{
//     pub fn apply(self, input: Column<T>) -> Column<T>{
//         match self{
//             Operations::Map(f) => {
//                 unary_engine::execute_unary(input, MapOp {predicate : f})
//             },
//             Operations::Filter(f) => {
//                 unary_engine::execute_unary(input, FilterOp {predicate : f})
//             },
//             Operations::Concat(other_input) => {
//                 let result = binary_engine::execute_binary(input, other_input, ConcatOp);
//                 result
//             }
//         }
//     }
// }

impl<T> fmt::Debug for LazyColumn<T>{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str_arr: Vec<&str> = vec![];
        for op in self.operation_list.iter().clone(){
            let result = match op{
                Operations::Map(_) => {
                    str_arr.push("Map");
                },
                Operations::Concat(_) => {
                    str_arr.push("Concat");
                },
                Operations::Filter(_) => {
                    str_arr.push("Filter");

                },
            };
        }
        f.debug_struct("operations").field("op", &str_arr).finish()
    }
}