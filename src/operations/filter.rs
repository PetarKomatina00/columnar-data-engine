
use crate::models::column::Column;
// impl<T: Clone> Column<T>{
//     fn filter_mask(&self, statement: fn(&T) -> bool) -> Vec<bool>{
//     self.data.iter().map(|v| statement(v)).collect()
// }
// fn apply_mask(mut self, mask: Vec<bool>) -> Column<T>{
//     let filtered: Vec<T> = self.data
//         .into_iter()
//         .zip(mask)
//         .filter_map(|(v, m)| if m == true {Some(v)} else {None})
//         .collect();
//     Column { data: filtered }
//     }
//     pub fn filter_column(mut self, statement: fn(&T) -> bool) -> Column<T>{
//         let mask = self.filter_mask(statement);
//         self.apply_mask(mask)
//     }
// }

use crate::traits::traits::UnaryTrait;
#[derive(Clone, Copy)]
pub struct FilterOp<F>{
    pub predicate: F
}
impl<T,F> UnaryTrait<T> for FilterOp<F>
where F: Fn(&T) -> bool
{
    type Output = T;

    fn execute(self, input: Column<T>) -> Column<Self::Output> {
        let data = input.data.into_iter().filter(|v| (self.predicate)(v)).collect();
        Column { data }
    }

}