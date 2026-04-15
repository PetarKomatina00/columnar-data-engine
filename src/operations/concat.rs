//     pub fn concat_columns<U>(self, data: Vec<U>) -> Column<String>
//         where 
//         T: Display,
//         U: Display{
//         if self.data.len() != data.len(){
//             panic!("Vectors must be same len");
//         }
//         let result: Vec<String> = self.data.iter().zip(data.iter())
//         .map(|(a,b)| format!("{} {}", a,b)).collect();
//         Column { data: result }
//     }
// }

// use std::fmt::Display;

// use crate::traits::operations::Operation;
// struct ConcatOp<F>{
//     predicate: F
// }

// impl<T,U,F> Operation<T> for ConcatOp<F> where 
// T: Display,
// U: Display
// {
//     type Output = String;
//     fn execute(self, input: crate::column::Column<T>) -> crate::column::Column<Self::Output> {
//         if 
//     }

// }

use std::fmt::Display;

use crate::traits::traits::BinaryTrait;
use crate::models::column::Column;
pub struct ConcatOp;

impl<T> BinaryTrait<T, T> for ConcatOp
where 
T: Display{
    type Output = T;

    fn execute(self, mut data_left: Column<T>, data_right: Column<T>) -> Column<Self::Output> {
        if data_left.data.len() != data_right.data.len(){
            panic!("Columns need to be the same length");
        }
        
        // let result: Vec<String> = data_left.data
        //     .into_iter()
        //     .zip(data_right.data.into_iter())
        //     .map(|(a,b)|format!("{} {}",a,b))
        //     .collect();

        data_left.data.extend(data_right.data);
        Column { data:  data_left.data}
    }
} 