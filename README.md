# assert2ify
A macro to replace standard library assertions against the assertions from the assert2 crate, which provide much better 
error messages.


TODO: mention that we should write
``` 
#[assert2ify(check)]
#[test]
#[should_panic(expected = "check failed")]
``` 
in this order for tests, otherwise clippy will get confused and complain about
unused attributes with `#[should_panic]`, because it
will not recognize that the test and should_panic attribute
go together