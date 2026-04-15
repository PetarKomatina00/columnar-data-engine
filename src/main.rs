mod models;
mod operations;
mod traits;
mod engine;
mod tests;
use std::iter::Filter;
use models::column::Column;
use crate::engine::binary_engine;
use crate::engine::unary_engine;
use crate::models::column::LazyColumn;
use crate::operations::concat::ConcatOp;
use crate::operations::filter;
use crate::operations::filter::FilterOp;
use crate::operations::map;
fn main() {

    let names = vec!["Petar", "Koma", "Nikolija", "Kosta"].to_vec();
    let last_names = vec!["Komatina", "Komatinovic", "Petrovic", "Vukasinovic"].to_vec();
    let mut col: Vec<i32> = vec![10,20,20,30,40,15];
    let mut numbers: Vec<i32> = vec![5, 5, 5, 5];
    //println!("{:?}", col.filter_column(|v| v > 15));

    let mut names = Column::new(names);
    let mut last_names = Column::new(last_names);
    // let result = column.filter_column(|v| v > "a");
    // println!("{:?}", result);
    // let result = column.filter_column(|v| v.contains("Ko"));
    // println!("{:?}", result);

    // let x = binary_engine::execute_binary(names.clone(), last_names.clone(), ConcatOp);
    // println!("{:?}", x);
    // let filter_op = FilterOp {predicate: |v: &&str| v.contains("Ko")};
    // let x = unary_engine::execute_unary(last_names, filter_op);
    //println!("{:?}", x);

    let result = LazyColumn::new(Column {data : col})
    .filter(|x| x % 2 == 0)
    .map(|x| x * 2)
    .concat::<i32>(Column { data: numbers });

    println!("{:?}", result);

    let result_new = result.collect();
    println!("{:?}", result_new);

    

    println!("Hello, world!");
}
