/*
Ownership is a set of rules that governs how a Rust program manages memory
Some languages have garbage collectors and some require the programmer
to explicitly allocate and free up memory.
Rust takes a third approach: ownership.
the compiler checks a set of rules related to memory management and won't compile
if any of them are broken

The main purpose of Ownership is to manage heap data (as opposed to stack data)

we'll be exploring ownership by working with strings
*/

pub fn run() {
    // Ownership rules:
    // 1. Each value in Rust has an owner
    // 2. There can only be one owner at a time
    // 3. When the owner goes out of scope, the value will be dropped

    // String is mutable, whereas &str is not, because String is not a fixed length
    // and so is stored on the heap, rather than the stack

    // Rust allocates memory for the String at runtime when we call ::from()
    // That memory is held on to, until the string variable goes out of scope
    // one it is out of scope Rust calls a function called 'drop' that frees up
    // memory from the now-out-of-scope variable. (basically calls it at a closing curly brace)
    let mut s = String::from("hello");
    println!("the original string was: {s}");
    s.push_str(", world!");
    println!("the mutated string is now: {s}");

    // Ways variables and data interact: Move
    // multiple variables can interact with the same data
    let mut x = 5;
    let y = x;
    println!("x = {x}");
    println!("y = x = {y}");
    x += 1;
    println!("x is now: {x}");
    println!("how does that affect y? y is: {y}");
    // string version
    // let s1 = String::from("hello");
    // let s2 = s1;
    // println!("s1 = {s1}"); // this doesn't compile, because s1 no longer holds the data, it was moved to s2
    // println!("s2 = s1 = {s2}");

    // Ways variables and data interact: Clone
    // this deeply copies the heap data, where move does not
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, and s2 cloned s1 and = {}", s1, s2);

    // Ownership and functions
    {
        let string = String::from("hello"); // string comes into scope
        takes_ownership(string); // strings's value moves into the function...
                                 // ...and so is no longer valid here
        let num = 5; // num comes into scope
        makes_copy(num) // num would move into the function,
                        // but i32 is Copy, so it's okay to still use x afterward
    } // Here, num goes out of scope, then string. But because string's value was moved, nothing
      // special happens

    fn takes_ownership(some_string: String) { // some_string comes into scope
        println!("{}", some_string);
    } // Here, some_string goes out of scope and `drop` is called. The backing
      // memory is freed.

    fn makes_copy(some_integer: i32) { // some_integer comes into scope
        println!("{}", some_integer);
    } // Here, some_integer goes out of scope. Nothing special happens.
}
