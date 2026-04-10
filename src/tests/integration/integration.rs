fn chain_unit_test() -> Vec<i32>{
    let input: Vec<i32> = vec![1, 2, 3, 4,5];

    let filtered: Vec<i32> = input
    .iter()
    .map(|v| v + 2)
    .filter(|v| v % 2 == 0)
    .chain(vec![10, 11])
    .collect();

    filtered
}

#[test]
fn integration_test(){
    let filtered = chain_unit_test();

    assert_eq!(filtered, vec![4,6,10,11]);
}
