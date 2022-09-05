// 5.1: Defining and Instantiating Structs
// Structs are like tuples where you name the values (they're objects)
// properties of a struct are called "fields"

pub fn _run() {
    //defined using the "struct" keyword and provide the field names and types
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    // now to use it we create instance of it by specifying values for the field
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // to get a specific value from a struct instance we use dot notation
    println!("The email field on user1 is: {}", user1.email);
    println!("The username field on user1 is: {}", user1.username);
    println!("The active field on user1 is: {}", user1.active);
    println!(
        "The sign_in_count field on user1 is: {}",
        user1.sign_in_count
    );

    // if the instance is mutable we can use dot notation to change field values as well
    user1.sign_in_count = 2;
    println!("the user1 sign in count is now: {}", user1.sign_in_count);

    // we can return new instances of structs from functions
    fn build_user(email: String, username: String) -> User {
        User {
            email: email,
            username: username,
            active: true,
            sign_in_count: 1,
        }
    }

    let _user2 = build_user(String::from("user2@example.com"), String::from("user2"));

    // Using the Field Init Shorthand
    // very similar to javascript, of a variable has the same name as a field you can use shorthand
    // lets redefine build_user with shorthand
    /*
    fn build_user(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1
        }
    }
    */

    // Creating Instances From Other Instances
    // use the 'struct update syntax'
    // first the regular way
    let user3 = User {
        active: true,
        email: String::from("user3@example.com"),
        username: String::from("user3"),
        sign_in_count: 1,
    };

    // now using the struct update syntax
    // we'll use user3 as our template and we only need to update the email and username
    let _user4 = User {
        email: String::from("user4@example.com"),
        username: String::from("user4"),
        ..user3 // kind of like a spread syntax, except 2 dots. specifies the remaining fields not explicitly set
    };
    // if we didn't set new values for the String type fields the data from those fields in
    // user 3 would have been moved to user4, meaning user3 would no longer have been useable afterwards

    // Using Tuple Structs without Named Fields to Create Different Types
    // You can create structs that look similar to tuples
    // example:
    struct Color (i32, i32, i32);
    struct Point (i32, i32, i32);

    let _black = Color(0, 0, 0,);
    let _origin = Point(0, 0, 0,);

    // Unit-Like Structs Without Any Fields
    // You can define structs that don't have any fields
    // useful when you need to implement a trait on some type
    // but don't have any data that you want to store in the type itself
    // more on traits in chapter 10
    struct AlwaysEqual;
    let _subject = AlwaysEqual; // later we'll learn about implementing some behavior for this
    
}
