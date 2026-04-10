use columnar_data_engine::engine::unary_engine;
use columnar_data_engine::models::column::Column;
use columnar_data_engine::operations::map::MapOp;



fn map_op<T: ::core::marker::Copy, U, F: ::core::marker::Copy>(input: Column<T>, op: MapOp<F>)
-> (Column<U>, Column<U>)
where F: Fn(&T) -> U{
    let temp_data = input.clone();
    let temp_op = op.predicate.clone();
    let result = unary_engine::execute(input, op);
    let test_result: Vec<U> = temp_data.data.iter().map(temp_op).collect();

    (result, Column {data: test_result})
    
}

#[cfg(test)]
mod map{
    use super::*;
    #[test]
    fn test_map_op(){
        let input = vec![1,2,3,4];
        let input_col = Column{data : input};
        let (result, test_result) = map_op(input_col, MapOp { predicate: |v: &i32| v + 2});
        
        assert_eq!(result, test_result);
    }

    #[test]
    fn test_map_op_empty_arr(){
        let (result, test_result) = map_op(Column {data : Vec::new()}, MapOp { predicate: |v: &i32| v + 2});
        assert_eq!(result, test_result);
    }
    #[test]
    #[should_panic]
    fn test_map_op_ed_max_size_arr(){
        let (result, test_result) = map_op(Column {data : vec![i32::MAX]}, MapOp { predicate: |v: &i32| v + 2});
        
        assert_eq!(result, test_result);
    }
}