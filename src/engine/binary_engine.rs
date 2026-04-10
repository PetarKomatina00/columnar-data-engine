
use crate::{models::column::Column, traits::traits::BinaryTrait};
pub fn execute_binary<T,U, Op>(
    data_left: Column<T>,
    data_right: Column<U>,
    predicate: Op
) -> Column<Op::Output>
where Op: BinaryTrait<T,U>{
    predicate.execute(data_left, data_right)
}