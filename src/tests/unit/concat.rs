
use columnar_data_engine::models::column::Column;
use columnar_data_engine::operations::concat::ConcatOp;
use columnar_data_engine::engine::binary_engine;

fn concat_op<'a>(names: Vec<&'a str>, last_names: Vec<&'a str>) -> Column<&'a str>{

    let col_names = Column {data : names};
    let col_last_names = Column {data : last_names};
        
    let result: Column<&'a str> = binary_engine::execute_binary(col_names, col_last_names, ConcatOp);

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

        let mut test_result: Column<&str> = Column {data: vec!["John Doe", "John Smith"]};

        let result = concat_op(names, last_names);
        assert_eq!(result, test_result);
    }

    #[test]
    #[should_panic(expected = "Columns need to be the same length")]
    fn test_concat_op_panic(){
        let names: Vec<&str> = vec!["John", "John"];
        let last_names: Vec<&str> = vec!["Doe"];

        let mut test_result: Column<&str> = Column {data: vec!["John Doe", "John Smith"]};

        let result = concat_op(names, last_names);

        assert_eq!(result, test_result);
    }

    #[test]
    fn test_concat_op_empty_arr(){

        let names: Vec<&str> = vec![];
        let last_names: Vec<&str> = vec![];

        let mut test_result: Column<&str> = Column { data: vec![] };

        let result = concat_op(names, last_names);
        assert_eq!(result, test_result);
    }

}