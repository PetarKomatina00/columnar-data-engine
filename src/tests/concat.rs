
use columnar_data_engine::models::column::Column;
use columnar_data_engine::operations::concat::ConcatOp;
use columnar_data_engine::engine::binary_engine;

fn concat_op(panic: bool) -> (Column<String>, Column<String>, ) {
    let mut names: Vec<&str> = vec!["John", "John"];
    let last_names: Vec<&str> = vec!["Doe", "Smith"];

    if panic{
        names.push("Alice");
    }

    let col_names = Column {data : names};
    let col_last_names = Column {data : last_names};
        
    let result = binary_engine::execute_binary(col_names, col_last_names, ConcatOp);

    let mut test_result: Column<String> = Column::new(Vec::new());
    test_result.data.push(format!("John Doe"));
    test_result.data.push(format!("John Smith"));

    (result, test_result)
}
#[cfg(test)]
mod concat{
    use columnar_data_engine::operations::concat;
    use crate::tests::concat::concat_op;
    #[test]
    fn test_concat_op(){
        let (result,test_result) = concat_op(false);
        assert_eq!(result, test_result);
    }

    #[test]
    #[should_panic(expected = "Columns need to be the same length")]
    fn test_concat_op_panic(){
        let (_result, _test_result) = concat_op(true);
    }
}