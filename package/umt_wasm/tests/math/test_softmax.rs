use umtn_wasm::softmax;

#[test]
fn test_softmax() {
    assert_eq!(softmax(vec![0.0, 0.0, 0.0],3), vec![0.333, 0.333, 0.333]);
    assert_eq!(softmax(vec![1.0, 1.0, 1.0],3), vec![0.333, 0.333, 0.333]);
    assert_eq!(softmax(vec![1.0, 2.0, 3.0],3), vec![0.090, 0.245, 0.665]);
    assert_eq!(softmax(vec![1.0, 2.0, 3.0, 4.0],3), vec![0.032, 0.087, 0.237, 0.644]);
    assert_eq!(softmax(vec![1.0, 2.0, 3.0, 4.0, 5.0],3), vec![0.012, 0.032, 0.086, 0.234, 0.636]);
}