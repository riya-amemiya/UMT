use umtn_wasm::factorize;

#[test]
fn test_factorize(){
    assert_eq!(factorize(1), vec![1]);
    assert_eq!(factorize(2), vec![2]);
    assert_eq!(factorize(10), vec![2, 5]);
}