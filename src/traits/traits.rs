
use crate::Column;
pub trait UnaryTrait<T> {
    type Output;

    fn execute(self, input: Column<T>) -> Column<Self::Output>;
}

pub trait BinaryTrait<T,U>{
    type Output;

    fn execute(self, data_left: Column<T>, data_right: Column<U>) -> Column<Self::Output>;
}