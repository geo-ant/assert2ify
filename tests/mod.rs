use assert2ify::assert2ify;

use assert2::assert as assert2;

#[test]
#[assert2ify]
fn my_test() {
    assert!(true);
    let v = vec![1,2,3];
    //todo! parse messages as well
    assert_eq!(v.len(),
               20,
               "these {} {} {}", 4,
               "things", "are now parsed");
}