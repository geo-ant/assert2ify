use assert2ify::assert2ify;

use assert2::assert as assert2;

fn result_func() -> Result<i32,String> {
    Ok(42)
}

#[test]
#[assert2ify]
fn my_test() {
    assert!(true);
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

    assert2!(let Err(_)   = result_func());
}