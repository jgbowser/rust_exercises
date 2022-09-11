// 9.3 To panic! or Not to panic!

// returning Result is a good default, because it allows for the possibility of
// recovering from an error, and can always still panic! if it needs to.

pub fn run() {
    // Cases in Which You Have More Information Than the Compiler
    // sometimes you know something won't fail, but the compiler doesn't
    // in these cases it makes sense to use expect/unwrap instead of dealing
    // with Resutls

    use std::net::IpAddr;

    let _home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");

    // Creating Custom Types for Validation
    // We can create custom types to take advantage of Rust's compiler to ensure
    // we have valid data flowing through our code

    pub struct _Guess {
        value: i32,
    }

    impl _Guess {
        // new ensures we have a valid value before creating a new Guess instance 
        // using the provided value
        pub fn _new(value: i32) -> _Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {}", value);
            }

            _Guess { value }
        }

        // the value field is private so that calling code can't set it directly.
        // this ensures it get validated because they must use Guess::new() to set
        // a value. that means we need a getter to get the private value back out,
        // hence our value function here.
        pub fn _value(&self) -> i32 {
            self.value
        }
    }

    // Now we can create Functions that take or return a Guess, instead of an i32
    // which ensures our value will always be between 1 and 100
}
