use crate::{engine::unary_engine, models::column::Column, operations::filter::FilterOp};
    fn filter_op<T: ::core::marker::Copy, F: ::core::marker::Copy>(input: Column<T>, op: FilterOp<F>)
    -> (Column<T>, Column<T>)
    where F: Fn(&T) -> bool{
        let temp_data = input.clone();
        let temp_op = op.clone();
        let result = unary_engine::execute_unary(input, op);
        let test_result = temp_data.data
            .into_iter()
            .filter(temp_op.predicate)
            .collect();

        (result, Column {data : test_result})
    }

#[cfg(test)]
mod filter{
    use std::iter::Filter;
    use crate::{engine::unary_engine, models::column::Column, operations::filter::FilterOp, tests::unit::filter::filter_op};
    #[test]
    fn test_filter_op(){

        let names: Vec<&str> = vec!["John", "Alice", "Bob"];
        let col_names = Column {data : names};

        let operation = FilterOp {predicate : |v: &&str| *v == "Alice"};
        let (result, test_result) = filter_op(col_names, operation);

        assert_eq!(result, test_result);
    }

    #[test]
    fn test_filter_op_pass_all(){
        let names: Vec<&str> = vec!["John", "John", "John"];
        let col_names = Column {data : names};

        let operation = FilterOp {predicate : |v: &&str| *v == "John"};
        let (result, test_result) = filter_op(col_names, operation);

        assert_eq!(result, test_result);
    }

    #[test]
    fn test_filter_op_deny_all(){
        let names: Vec<&str> = vec!["John", "Alice", "Bob"];
        let col_names = Column {data : names};

        let operation = FilterOp {predicate : |v: &&str| *v == "Doe"};
        let (result, test_result) = filter_op(col_names, operation);

        assert_eq!(result, test_result);
    }
}