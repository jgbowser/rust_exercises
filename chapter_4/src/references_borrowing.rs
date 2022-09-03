// References and Borrowing

// We don't want to have to pass through a value every time we use it in a function
// but still want access to it later. Instead, we can provide a reference to a value.

pub fn _run() {
    // using reference instead of taking ownership
    // the '&' represent references to the variables (s1 and the function signature)
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of {} is {}.", s1, len);

    fn calculate_length(s: &String) -> usize {
        // s is a reference to a String
        s.len()
    } // Here, s goes out of scope. But because it does not have ownership of what
      // it refers to, it is not dropped.
      // The action of creating a reference is called 'borrowing'

    // What happens if we try to modify something we are borrowing?
    // let s = String::from("hello");
    // change(&s);

    // fn change(some_string: &String) {
    //     some_string.push_str(", world!");
    // }
    // just like variables, references are immutable by default
    // we can fix this though using mutable references
    let mut s = String::from("hello");
    change(&mut s);
    println!("We changed s from 'hello' to '{s}' via mutable references");
    fn change(some_string: &mut String) {
        some_string.push_str(", world!");
    }

    // can only ever have a single mutable reference to a value at a time
    // the code below will fail to compile
    /*
    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s;
    println!("{}, {}", r1, r2);
    */
    // this works though if we mess around with the scope a bit...
    let mut str = String::from("hello");
    {
        let r1 = &mut str;
        println!("r1 = {r1}");
    } // r1 goes out of scope here, so new references are fine after this
    let r2 = &mut str;
    println!("r2 = {r2}");

    // mixing mutable/immutable for a single value is not allowed as well
    // the following code will fail to compile
    /*
    let mut s = String::from("hello");

    let r1 = &s; <- regular reference
    let r2 = &s; <- still good
    let r3 = &mut s; <- annnnd it broke
    println!("{}, {}, and {}", r1, r2, r3);
    */

    // with scoping though, we can have what looks like a mix, even if it really isn't
    let mut string = String::from("Hello");

    let ref1 = &string;
    let ref2 = &string;
    println!("{} and {}", ref1, ref2); //ref1 and ref2 are not used after this, so they are now out of scope...

    let ref3 = &mut string;
    println!("{}", ref3);

    // Dangling References
    // Dangling pointers occur when a pointer references a place in memory that has been given to something else
    // Rust prevents this from happening
    // the code below fails to compile for this reason
    /*
    let reference_to_nothing = dangle();
    fn dangle() -> &String { --> dangle returns a reference to a String
        let s = String::from("hello") --> s is a new String
        &s --> We return a reference to the String
    } --> s goes out of scope and is dropped. it's memory goes away too
    */
}
