
use crate::{models::column::Column, traits::traits::UnaryTrait};
pub fn execute<T, U>(column: Column<T>, op: U) -> Column<U::Output>
where
    U: UnaryTrait<T>,
{
    op.execute(column)
}