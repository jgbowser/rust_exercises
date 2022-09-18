// 10.3 Validating References with Lifetimes

/*
we've already been using Lifetimes uo until now without even
realizing it. Lifetimes means we ensure that that references
are valid as long as we need them to be.

Every reference in Rust has a lifetime, which is the scope for
which that reference is valid. Most of the time lifetimes are
implicit and inferred, just like types. We only annotate types
when multiple types are possible. In a similar way, we must
annotate lifetimes when the lifetimes of references could be
related in a few different ways. Rust requires us to annotate
the relationships using generic lifetime parameters to ensure
the actual references used at runtime will definitely be valid.

This is not even a concept most other programming languages have.
Following, we'll review common ways that we'll encounter lifetime
syntax to get comfortable with the concept.
*/

pub fn run() {
    // Preventing Dangling References with Lifetimes
    /*
    the main aim of lifetimes is to prevent dangling references, which cause a
    program to reference data other than the data it is intended to reference
    
    the following code will not compile because the value of x does not live long
    enough (not in scope) after being referenced in r, to then be read later
    */

    // let r;
    // {
    //     let x = 5;
    //     r = &x;
    // }
    // println!("r: {}", r);

    // Rust knows this code is invalid because it uses a "Borrow Checker"

    // The Borrow Checker

    /*
    The "borrow checker" in the Rust compiler compares scopes to determine whether
    all borrows are valid. It basically looks at the size of each scope, and references
    amongst them and fails when it finds that a reference applies to a smaller scope.
    We can fix the code above like so:
    */

    let x = 5;
    let r = &x;
    println!("r: {}", r);

    // Generic Lifetimes in Functions
    /*
    To start we'll write a function that returns the longest of 2 provided string slices
    */
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("the longest string is {}", result);
    /*
    Note that the function takes string slices (which are references) rather than
    strings, because we don't want the function to take ownership
    the implementation of `longest` below won't compile
    */

    // fn longest(x: &str, y: &str) -> &str {
    //     if x.len() > y.len() {
    //         x
    //    } else {
    //     y
    //    }
    // }

    /*
    We get a "missing lifetime specifier" error. We need to add a generic lifetime
    parameter on the return type because Rust can't tell whether the reference being
    returned refers to x or y (neither do we actually). The fix:
    */

    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
                    x
               } else {
                y
               }
    }

    // Lifetime Annotation Syntax
    /*
    Lifetime syntax annotation describes the relationships of lifetimes of multiple
    references to each other, without affecting the lifetimes. Just as functions
    can accept any type when the signature specifies a generic type param, functions
    can accept references with any lifetime by specifying a generic lifetime parm.

    The syntax is slightly unusual. The names start with "'" and are usually all 
    lower case, and very short. most people use `'a` as their first lifetime param
    Some example references:

    &i32         // a reference
    &'a i32      // a reference with an explicit lifetime
    &'a mut i32  // a mutable reference with an explicit lifetime
    */

    // Lifetime Annotations in function Signatures
    
}