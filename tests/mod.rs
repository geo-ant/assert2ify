use assert2ify::assert2ify;

use assert2::assert as assert2;

#[test]
#[assert2ify]
fn my_test() {
    assert!(true);
    assert_eq!(10,20);
}