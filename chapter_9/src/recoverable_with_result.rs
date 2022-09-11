// 9.2 Recoverable Errors with Result

// most errors aren't serious enough to require the program to crash, sometimes
// failures are expected and can be easily handled

pub fn _run() {
    /*
    remember the Result type looks like:
    enum Result<T, E> {
        Ok(T),
        Err(E)
    }

    where T = type of value returned on success and E is the type of error
    */

    // lets call a function that returns a Result value
    /*
    use std::fs::File;

    let greeting_file_result = File::open("hello.txt");
    // we now have a Result, time to handle the possibilities
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    println!("{:?}, this won't print because we'll panic first", greeting_file);
     */

     // Matching on Different Errors
     use std::fs::{self, File};
     use std::io::ErrorKind;

    let greeting_file_result = File::open("hello.txt");
    // we now have a Result, time to handle the possibilities
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
    println!("{:?}, this won't print because we'll panic first", greeting_file);

    /*
    A more concise way to write this using closures and other methods
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    */

    // Shortcuts for Panic on Error: unwrap and expect
    // unwrap is a Result helper method. if the result is Ok, unwrap returns that value
    // if it is Err, it calls the panic! macro for us.
    // example:
    // let farewell_file = File::open("goodbye.txt").unwrap();
    // expect is similar, but lets us provide the panic message
    /*
    let farewell_file = File::open("goodbye.txt")
        .expect("goodbye.txt should be included in the project");
    */

    // Most will choose to use `expect` over `unwrap` because more context can
    // be provided when failures do occur

    // Propagating Errors
    use std::io::{self, Read};

    // this function will either return an Ok Result, or an Error that the calling
    // code can then handle as it wishes
    fn _read_username_from_file() -> Result<String, io::Error> {
        let username_file_result = File::open("username.txt");

        let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut username = String::new();
        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        }
    }

    // this is a fairly common pattern, and so Rust provides some syntax to make
    // it more concise, the `?` operator.

    // A Shortcut for Propagating Errors: the `?` Operator
    // the function above can be re-written as:
    fn _read_username_from_file_short() -> Result<String, io::Error> {
        let mut username_file = File::open("username.txt")?;
        let mut username = String::new();
        username_file.read_to_string(&mut username)?;
        Ok(username)
    }

    // The `?` at the end of lines that result in a Result type is basically the
    // same as the match expressions in the initial implementation.
    // One difference is that Errors returned out via the `?` operator go through
    // the `from` function, which converts values from 1 type to another. So in
    // this case it will convert whatever error type we receive into the error
    // type that is returned according to the function signature.

    // but wait, we can condense this even more using chaining
    fn _read_username_from_file_shorter() -> Result<String, io::Error> {
        let mut username = String::new();

        File::open("username.txt")?.read_to_string(&mut username)?;

        Ok(username)
    }

    // shorter still...
    fn _read_username_from_file_shortest() -> Result<String, io::Error> {
        fs::read_to_string("username.txt")
        // this is a built in that does all the functionality in the previous fns
    }

    // Where the `?` Operator Can Be Used
    // can only be used in functions whose return type is compatible with the
    // value that `?` is used on, because `?` may perform an early return.
    // So `?` can only be used in functions that return Result, Option,
    // or another type that implements FromResidual

}