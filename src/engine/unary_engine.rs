use crate::Column;
use crate::traits::traits::UnaryTrait;
fn execute<T, U>(column: Column<T>, op: U) -> Column<U::Output>
where
    U: UnaryTrait<T>,
{
    op.execute(column)
}