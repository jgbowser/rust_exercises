// 11.1 How to Write Tests

// Tests are functions with the test attribute. We've defined attributes before
// like: derive

#[cfg(test)]
mod tests {
    use super::*;
    #[test] // always use this to mark test functions (eg: not a setup function)
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
    // run with `cargo test`

    // # [test]
    // fn another() {
    //     panic!("Make this test fail");
    // }

    // Checking Results with the assert! Macro

    /*
    The assert macro, provided by the standard library, is useful when you want to
    ensure that some condition in a test evaluates to true. we give the assert! macro
    an argument that evaluates to a Boolean. If the value is true, nothing happens
    and the test passes. If the value is false, assert! macro calls panic! to cause
    the test to fail
    */

    // because can_hold returns a boolean, it is a good use case for assert!

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(!smaller.can_hold(&larger));
    }

    // Testing Equality with the assert_eq! and assert_ne! Macros
    /*
    Where assert! tests the truthy-ness of a result, assert_eq! and assert_ne! test whether
    2 values are or are not equivalent.
    */

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    // Adding Custom Failure Messages
    // custom messages that will be displayed on failure can be added as optional
    // args to the assert macros. Any args after the required ones are passed along
    // to format!, this means you can pass in placeholders and variables to view
    // actual results

    #[test]
    fn greeting_contains_name() {
        let result = greeting("John");
        assert!(
            result.contains("John"),
            "Greeting did not contain the name, value was {}",
            result
        );
    }

    // Checking for panics with should_panic
    /*
    in addition to checking return values it is important to check that our code
    handles error conditions as expected. Example: consider he Guess type we created
    in Chapter 9. Other code that uses Guess depends on the guarantee that Guess
    instances will contain values between 1 and 100. Let's test that a value outside
    of that range correctly panics
    */

    # [test]
    // # [should_panic] // this gets triggered for any panic, we can be more specific
    # [should_panic(expected = "greater than 100")] //notice it can be a substring
    fn greater_than_100() {
        Guess::new(200);
    }

    // Using Result<T, E> in Tests
    /*
    Our tests so far all panic when they fail. We can also write tests that use
    Result<T, E>
    */

    # [test]
    fn it_works_again() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    /*
    Writing tests so they return a Result<T, E> enables you to use the question
    mark operator in the body of tests, which can be a convenient way to write tests
    that should fail if any operation within them returns an Err variant.

    You can’t use the #[should_panic] annotation on tests that use Result<T, E>. To
    assert that an operation returns an Err variant, don’t use the question mark
    #operator on the Result<T, E> value. Instead, use assert!(value.is_err()).
    */
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    // format!("Hello {}!", name) correct version
    String::from("Hello") // buggy
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        // if value < 1 || value > 100 {
        //     panic!("Guess value must be between 1 and 100, got: {}", value);
        // } correct version

        // if value < 1 {
        //     panic!("This is broken!!!")
        // } buggy

        if value < 1 {
            panic!("The value provided was less than 1, got: {}", value);
        } else if value > 100 {
            panic!("The value provided was greater than 100, got: {}", value)
        }

        Guess { value }
    }
}
