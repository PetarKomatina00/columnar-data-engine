
use columnar_data_engine::models::column::Column;
use columnar_data_engine::operations::concat::ConcatOp;
use columnar_data_engine::engine::binary_engine;

fn concat_op(names: Vec<&str>, last_names: Vec<&str>) -> Column<String>{

    let col_names = Column {data : names};
    let col_last_names = Column {data : last_names};
        
    let result = binary_engine::execute_binary(col_names, col_last_names, ConcatOp);

    result
}
#[cfg(test)]
mod concat{
    use crate::tests::unit::concat::concat_op;

    use super::*;
    #[test]
    fn test_concat_op(){
        let mut names: Vec<&str> = vec!["John", "John"];
        let last_names: Vec<&str> = vec!["Doe", "Smith"];

        let mut test_result: Column<String> = Column::new(Vec::new());
        test_result.data.push(format!("John Doe"));
        test_result.data.push(format!("John Smith"));

        let result = concat_op(names, last_names);
        assert_eq!(result, test_result);
    }

    #[test]
    #[should_panic(expected = "Columns need to be the same length")]
    fn test_concat_op_panic(){
        let names: Vec<&str> = vec!["John", "John"];
        let last_names: Vec<&str> = vec!["Doe"];

        let mut test_result: Column<String> = Column::new(Vec::new());
        test_result.data.push(format!("John Doe"));
        test_result.data.push(format!("John Smith"));

        let result = concat_op(names, last_names);

        assert_eq!(result, test_result);
    }

    #[test]
    fn test_concat_op_empty_arr(){

        let names: Vec<&str> = vec![];
        let last_names: Vec<&str> = vec![];

        let mut test_result: Column<String> = Column::new(Vec::new());

        let result = concat_op(names, last_names);
        assert_eq!(result, test_result);
    }

}