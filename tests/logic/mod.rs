//! Test that the basic logic of the assertions works.
//! These tests are cribbed and slightly modified versions of the tests inside the
//! assert2 crate

#![allow(clippy::eq_op)]
#![allow(clippy::op_ref)]
#![allow(clippy::assertions_on_constants)]
#![allow(unused_attributes)]

mod helper_macros;
/// Helper macro that takes a test and makes 3 variants
/// one "normal variant", one with #[assert2ify] config
/// and one with an #[assert2ify(check)] config
/// All tests should either panic or none of them, so we save
/// some manual labor
#[macro_export]
macro_rules! test_all_assertification_styles {
    (
        #[test]
        $(#[$attr:meta])?
        fn $test_name:ident () {
            $($body:tt)+
        }
    ) => {
        ::paste::paste! {
            #[test]
            $(#[$attr])?
            fn $test_name () {
                $($body)+
            }

            #[test]
            $(#[$attr])?
            #[::assert2ify::assert2ify]
            fn [< $test_name _with_assertification>] () {
                $($body)+
            }

            #[test]
            $(#[$attr])?
            #[::assert2ify::assert2ify(check)]
            fn [< $test_name _with_checkification>] () {
                $($body)+
            }
        }
    };

    (
        fn $test_name:ident () {
            $($body:tt)+
        }
    ) => {compile_error!("The given function must be marked #[test]");};
}

/// create multiple (assertify, checkify, normal) version of a test that should panic
macro_rules! test_should_panic {
    ($test_name:ident, $body:expr) =>  {
        test_all_assertification_styles! {
            #[test]
            #[should_panic]
            fn $test_name () {
                $body;
            }
        }
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd)]
struct I(i32);

// a simple test case, which is copied from the assert2 test cases
test_all_assertification_styles! {
    #[test]
    fn assert_pass() {
        assert!(1 == 1);
        assert_eq!(1, 1);
        assert_ne!(1, 2);


        assert!(1 == 1, "{}", "math broke");
        assert_eq!(1, 1, "{}", "math broke");
        assert_ne!(1, 2, "{}", "math broke");

        assert!(true && true);
        assert!(true == true);
        assert_eq!(true, true);
        assert_ne!(false, true);

        assert!(true && true, "{}", "logic broke");
        assert!(true == true, "{}", "logic broke");
        assert_eq!(true, true, "{}", "logic broke");
        assert_ne!(true, false, "{}", "logic broke");

        assert!(matches!(Result::<i32, i32>::Ok(10),Ok(10) ));
        assert!(matches!(Result::<i32, i32>::Ok(10),Ok(10)), "{}", "rust broke");
    }
}

// a simple test case, which is copied from the assert2 test cases
// makes sure that non sized data can be used in assertions
test_all_assertification_styles! {
    #[test]
    fn non_sized() {
        assert!(b"hello"[..] == b"hello"[..]);
        assert_eq!(b"hello"[..] , b"hello"[..]);
    }
}

test_should_panic!(panic_assert1, assert!(1 == 2));
test_should_panic!(panic_assert2, assert!(1 == 2, "{}", "math broke"));
test_should_panic!(panic_assert3, assert!(true && false));
test_should_panic!(panic_assert4, assert!(true && false, "{}", "logic broke"));
test_should_panic!(panic_assert5, assert!(matches!(Result::<i32, i32>::Err(10),Ok(_))));
test_should_panic!(panic_assert6, assert!(matches!(Result::<i32, i32>::Err(10),Ok(_)), "{}", "rust broke"));
test_should_panic!(panic_assert7, assert_eq!(true , false, "{}", "logic broke"));
test_should_panic!(panic_assert8, assert_ne!(1 , 1, "{}", "math broke"));


test_all_assertification_styles! {
    #[test]
    fn debug_refs() {
        assert_eq!(&1 , &1);
        assert_eq!(&&1 , &&1);
        assert_eq!(&&&&&&&1 , &&&&&&&1);

        assert_ne!(&1 , &2);
        assert_ne!(&&1 , &&2);
        assert_ne!(&&&&&&&2, &&&&&&&1);

        assert!(&1 == &1);
        assert!(&&1 == &&1);
        assert!(&&&&&&&1 == &&&&&&&1);
        assert!(matches!(&10,10));
        assert!(matches!(& &10,10));
    }
}
test_all_assertification_styles! {
    #[test]
    fn non_debug_refs() {
        assert!(&I(1) == &I(1));
        assert!(&&I(1) == &&I(1));
        assert!(&&&&&&&I(1) == &&&&&&&I(1));
        assert!(matches!(&I(10),I(10)));
        assert!(matches!(& &I(10),I(10)));
    }
}


test_all_assertification_styles! {
    #[test]
    fn no_copy() {
        let a = String::new();
        let b = String::new();
        assert!(a == b);
        assert!(a == b);
        assert_eq!(a,b);
        assert_eq!(a,b);
        drop(a);
        drop(b);
    }
}