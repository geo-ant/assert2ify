use assert2ify::assert2ify;
mod logic;

#[test]
#[should_panic(expected = "check failed")]
#[assert2ify(check, crate = assert2ify)]
pub(crate) fn test_assertion_is_replaced_in_nested_code() {
    let v = vec![1, 2, 3];
    for _ in 1..10
    {
        if 20 > 19
        {
            assert_ne!(v.len(),
                       3,
                       "these {} {} {}", 1,
                       "message(s)", "are now parsed");
        }
    }
}

#[test]
#[assert2ify(check)]
fn my_test2() {
    //::std::assert!(true);
    let v = vec![1, 2, 3];
    //todo! parse messages as well

    for _ in 1..10
    {
        if 20 > 19
        {
            assert!(v.len() > 20);
            assert!(v.len() < 2);
        }
    }
}

fn something() -> Result<i32, String> {
    Ok(1337)
}

fn error() -> Result<i32, String> {
    Err("bla".to_string())
}


test_all_assertification_styles! {
    #[test]
    fn test_assert_matches() {
        for _ in 1..10 {
            assert!(matches!(error(), Err(s) if s.contains("some")  ), "somethin {}", "is wrong");
        }
        assert!(matches!(something(), Err(s) if s.contains("some")  ), "somethin {}", "is wrong");
    }
}

#[test]
#[assert2ify]
fn test_nest() {
    identity!(
        identity!(
            identity!(
                assert_eq!(2,3))));
}

// this assertion will not be replaced,k
// but it will still exist and produce the
// expected panic
// the reason is that the folding
#[test]
#[assert2ify(check)]
fn test_nest_check() {
    identity!{
        identity!{
            identity!{
                let _a = 1+1;
                assert_eq!(2,3);}}};
}