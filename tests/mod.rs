use assert2ify::assert2ify;


#[test]
#[assert2ify]
fn my_test() {
    //::std::assert!(true);
    let v = vec![1,2,3];
    //todo! parse messages as well

    for _ in 1..10
    {
        if 20> 19
        {
            assert_eq!(v.len(),
                       20,
                       "these {} {} {}", 1,
                       "message(s)", "are now parsed");
        }
    }

    //assert2!(let Err(Some(_))   = result_func());
}

#[test]
#[assert2ify(check)]
fn my_test2() {
    //::std::assert!(true);
    let v = vec![1,2,3];
    //todo! parse messages as well

    for _ in 1..10
    {
        if 20> 19
        {
            assert!(v.len()>20);
            assert!(v.len()<2);
        }
    }
}

use assert2::assert as assert2;
use assert2::let_assert;

#[test]
//#[assert2ify(check)]
fn my_test3() {
    //::std::assert!(true);

    assert2!(let Ok(_) | Err(1) = Vec::<i32>::new().first().ok_or(&1));
}

