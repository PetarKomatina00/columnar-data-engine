use std::fmt::Display;

use crate::column::Column;

// impl<T> Column<T>{
//     pub fn map_column<U, F>(self, statement: F) -> Column<U>
//     where 
//     F: Fn(&T) -> U{
//         let data = self.data.iter().map(|v| statement(v)).collect();

//         Column { data }
//     }
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

use crate::traits::traits::UnaryTrait;

struct MapOp<F>{
    predicate: F
}
impl<T,U,F> UnaryTrait<T> for MapOp<F>
where F: Fn(T) -> U{
    type Output = U;
    fn execute(self, input: Column<T>) -> Column<Self::Output> {
        let data: Vec<U> = input.data.into_iter().map(|v| (self.predicate)(v)).collect();
        Column { data }
    }
}