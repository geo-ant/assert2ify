use assert2ify::assert2ify;

mod logic;

#[assert2ify(check, crate = assert2ify)]
#[test]
#[should_panic(expected = "check failed")]
pub(crate) fn assertion_is_replaced_in_nested_code() {
    let v = vec![1, 2, 3];
    for _ in 1..10
    {
        if 20 > 19
        {
            assert_ne!(v.len(),
                       3,
                       "these {} {} {}", 3,
                       "message(s)", "are now parsed");
        }
    }
}

#[assert2ify(check)]
#[test]
#[should_panic(expected = "the assertion is indeed replaced by check and does not panic")]
fn checkification_really_replaces_assertions_by_checks_that_do_not_immediately_panic() {
    //::std::assert!(true);
    let v = vec![1, 2, 3];
    assert!(v.len() > 20);
    assert!(true == false);
    panic!("the assertion is indeed replaced by check and does not panic");
}

fn something() -> Result<i32, String> {
    Ok(1337)
}

fn error() -> Result<i32, String> {
    Err("bla".to_string())
}

#[assert2ify(check)]
#[test]
#[should_panic]
// there is no check(let...) so even if checkification is enabled, then
// assert!(matches!(...)) is converted to assert2::assert(let ...))
// and fails immediately
fn test_that_checkification_replaces_assert_matches_with_assertions_instead() {
    assert!(matches!(something(), Err(_)), "something {}", "is wrong");
    panic!("this panic should never be reached, even for checkification");
}

test_all_assertification_styles! {
    #[test]
    #[should_panic]
    fn if_clauses_in_match_expressions_are_correctly_processed_when_condition_is_false() {
        assert!(matches!(error(), Err(s) if s.contains("foo")), "something {}", "is wrong");
    }
}

test_all_assertification_styles! {
    #[test]
    fn if_clauses_in_match_expressions_are_correctly_processed_when_condition_is_true() {
        assert!(matches!(something(), Ok(i) if i < 2000), "something {}", "is wrong");
    }
}

test_all_assertification_styles! {
    #[test]
    #[should_panic]
    fn nested_assertion_in_expression_inside_macro_is_replaced() {
    identity!(
        identity!(
            identity!(
                assert_eq!(2,3))));
    }
}

#[assert2ify(check)]
#[test]
#[should_panic(expected = "check failed")]
fn checkification_works_for_nested_assertion_in_expression_inside_macro() {
    identity!(
        identity!(
            identity!(
                assert_eq!(2,3))));
}


// this assertion will not be replaced, because my macro parsing
// only works when a macro contains an expression. But we make sure
// that the original assertion is carried through and the test still
// fails as expected.
test_all_assertification_styles! {
    #[test]
    #[should_panic]
    fn even_if_nested_assertions_inside_macros_are_not_replaced_they_still_fail_as_expected() {
        identity! {
            identity!{
                identity!{
                    let _a = 1+1;
                    assert_eq!(2,3);}}}
        ;
    }
}