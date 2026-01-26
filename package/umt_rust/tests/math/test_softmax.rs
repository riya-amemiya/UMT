use umt_rust::math::umt_softmax;

#[test]
fn test_softmax() {
    assert_eq!(
        umt_softmax(vec![0.0, 0.0, 0.0], 3),
        vec![0.333, 0.333, 0.333]
    );
    assert_eq!(
        umt_softmax(vec![1.0, 1.0, 1.0], 3),
        vec![0.333, 0.333, 0.333]
    );
    assert_eq!(
        umt_softmax(vec![1.0, 2.0, 3.0], 3),
        vec![0.09, 0.245, 0.665]
    );
    assert_eq!(
        umt_softmax(vec![1.0, 2.0, 3.0, 4.0], 3),
        vec![0.032, 0.087, 0.237, 0.644]
    );
    assert_eq!(
        umt_softmax(vec![1.0, 2.0, 3.0, 4.0, 5.0], 3),
        vec![0.012, 0.032, 0.086, 0.234, 0.636]
    );
    assert_eq!(
        umt_softmax(vec![-1.0, -2.0, -3.0], 3),
        vec![0.665, 0.245, 0.09]
    );
    assert_eq!(umt_softmax(vec![1.0, 2.0, 3.0], 2), vec![0.09, 0.24, 0.67]);
    assert_eq!(umt_softmax(Vec::<f64>::new(), 3), Vec::<f64>::new());
}
