use assert2_ify_macros::assert2_ify;

#[test]
#[assert2_ify]
fn my_test() {
    assert!(true);
    assert_eq!(10,20);
}