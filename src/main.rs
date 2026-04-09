mod column;
mod operations;
mod traits;
mod engine;
use column::Column;

use crate::engine::{binary_engine, unary_engine};
use crate::operations::filter;
use crate::operations::map;
fn main() {

    let names = vec!["Petar", "Koma", "Nikolija", "Kosta"].to_vec();
    let last_names = vec!["Komatina", "Komatinovic", "Petrovic", "Vukasinovic"].to_vec();
    let mut col = vec![10,20,20,30,40,15];
    //println!("{:?}", col.filter_column(|v| v > 15));

    let mut names = Column::new(names);
    let mut last_names = Column::new(last_names);
    // let result = column.filter_column(|v| v > "a");
    // println!("{:?}", result);
    // let result = column.filter_column(|v| v.contains("Ko"));
    // println!("{:?}", result);
    
    println!("Hello, world!");
}
