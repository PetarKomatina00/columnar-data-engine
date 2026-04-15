
use crate::{models::column::Column, traits::traits::UnaryTrait};
pub fn execute_unary<T, U>(column: Column<T>, op: U) -> Column<U::Output>
where
    U: UnaryTrait<T>,
{
    op.execute(column)
}