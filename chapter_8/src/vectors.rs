// 8.1 Storing Lists of Values with Vectors
// Vec<t> : store multiple values in a single structure that puts all the values
// next to each other in memory

pub fn run() {
    // Creating a New Vector
    let v: Vec<i32> = Vec::new(); // providing type annotation here because we didn't provide initial values.
    println!("our vector created without initial values: {:?}", v);
    // Normally Rust can infer this though when values are provided. Use the vec! macro
    let v2 = vec![1, 2, 3];
    println!("our vector created with initial values: {:?}", v2);

    // Updating a vector
    // we add values to a vector using the push method
    let mut v3 = Vec::new(); // don't need the annotation because Rust infers it from the code below
    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);
    println!("we pushed these values into this vector: {:?}", v3);

    // Reading Elements of Vectors
    // 2 ways: indexing or `get` method

    // indexing
    let v4 = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v4[2];
    println!("the third element of {:?} is {third}", v4);

    // get method
    let third: Option<&i32> = v4.get(2);
    match third {
        Some(third) => println!("the third element is {}", third),
        None => println!("there is no third element..."),
    }

    // these different methods allow us to tailor the behavior of the program
    // when we try accessing a non-existent index
    let v5 = vec![1, 2, 3, 4, 5];
    // let does_not_exist = &v5[100]; // <-- this panics! Best used when you want the program to crash if a non-existent index is referenced
    let _does_not_exist = v5.get(100); // <-- this doesn't panic, it just returns a `None` value. This approach is useful if accessing out of bounds indexes may happen, and shouldn't result in a crash.
    // there should then be some logic to handle the Some or None case

    // don't forget about the borrowing rules. This snippet fails to compile
    /*
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0]; <-- immutable borrow

    v.push(6); <-- mutable borrow

    println!("The first element is: {}", first); <-- another immutable borrow
    */

    // This seems a bit weird at first glance since we're only getting a reference.
    // but remember that vector values are all stored next to each other. This means
    // that adding a value could result in the vector's location in memory changing
    // to allow for the now longer list of values to be stored side-by-side

    // Iterating Over the Values in a Vector

    // using a for loop to get immutable references:
    let v6 = vec![100, 32, 57];
    for i in &v6 {
        println!("{i}");
    }

    // iterate over mutable references and make changes
    let mut v7 = vec![100, 32, 57];
    println!("v7 before the loop: {:?}", v7);
    for i in &mut v7 {
        *i += 50; // <-- * is the dereference operator and will be discussed more in Chapter 15
    }
    println!("v7 after the loop: {:?}", v7);

    // Using an enum to Store Multiple Types
    // Vectors can only hold a single type for all values. There are definitely 
    // cases where we want to be able to store a variety of types in a single vector though.
    // Variants of an enum are all under the same enum type though

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];
    println!("a vector with multiple types, thanks to enums: {:?}", row);

    // if you don't know the exhaustive list of types before runtime you would
    // want to use a Trait (more on that in Chapter 17)

    // there are plenty more vector methods other than just push. there's also
    // pop, to return the last element, amongst many others
}