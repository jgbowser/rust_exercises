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

use std::fmt::Display;

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
    returned refers to x or y (neither do we actually).
    */

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
    /*
    To use lifetime annotations in the function signatures we need to declare
    the generic lifetime parameter inside angle brackets between the function
    name and the parameter list, just like generic type parameters

    the signature should express the following constraint: the returned reference
    will be valid as long as both the parameters are valid. This is the relationship
    between lifetimes of the parameters and the return value. We name the lifetime
    `'a` and add it to each reference.
    The fix for our code that won't compile above:
    */

    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    /*
    we're basically defining the lifetime requirements of anything passed into this
    function. We aren't changing the lifetimes of anything passed in, we are
    merely letting the Rust compiler know that anything passed to this function
    must meet the lifetime requirements we've defined.

    When we pass concrete references to longest, the concrete lifetime that is
    substituted for 'a is the part of the scope of x that overlaps with the scope
    of y. In other words, the generic lifetime 'a will get the concrete lifetime that
    is equal to the smaller of the lifetimes of x and y. Because we’ve annotated
    the returned reference with the same lifetime parameter 'a, the returned reference
    will also be valid for the length of the smaller of the lifetimes of x and y.

    below are some examples of longest with a variety of lifetimes. So valid, some not
    */

    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("the longest string is {}", result);
    }
    /*
    all references passed to longest in the previous example will last at least as long as
    the shortest one, meaning all references will always be available long enough
    for this code to compile
    */

    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("the longest string is {}", result);

    /*
    in this example it won't compile because string2 won't be valid long enough
    even though looking at the code we know string1 would be the result, and so
    this actually wouldn't cause any problems, but we've restricted what the
    borrow checker will accept because of the use of the lifetime syntax
    */

    // Thinking in Terms of Lifetimes

    /*
    The way in which we specify lifetimes depends on what the function is doing.
    if longest just always returned the first parameter, then there would be no
    need to specify a a lifetime parameter for y

    fn longest<'a>(x: &'a str, y: &str) -> &'a str {
        x
    }

    the return value must match the lifetime of the lifetime parameter, or a value
    created within the function
    */

    // Lifetime Annotations in Struct Definitions

    /*
    So far, all the structs we've defined all hold owned types. We can define
    structs that hold references, but in that case we would need to add a lifetime
    annotation on every reference in the struct's definition
    */

    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    let novel = String::from("Call me Ishmael. Some year ago...");
    let first_sentence = novel.split(".").next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    println!("{}", i.part);

    // Lifetime Elision

    /*
    We've written function before this point that accepted references, but we never
    specified lifetimes. This is because the compiler uses something called
    "lifetime elision rules" which are used by the compiler to match certain patterns.
    In early Rust you always had to declare lifetimes, but it was repetitive, and
    many times followed the same exact patterns. This is where the elision rules
    came from. Basically the compiler will apply each of the rules to a fn definition
    and if at the end there are still uncertainties, it will not compile until we
    add lifetime annotations.
    */

    // Lifetime Annotations in Method Definitions

    /*
    often times lifetime annotations aren't required in method definitions because
    of the elision rules.

    If a struct has lifetime names, they must be used with impl because it is part
    of the struct's type
    */

    impl<'a> ImportantExcerpt<'a> {
        fn level(&self) -> i32 {
            3
        }
    }
    println!("{}", i.level());

    impl<'a> ImportantExcerpt<'a> {
        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("Attention please: {}", announcement);
            self.part
        }
    }
    i.announce_and_return_part("I'm hungry");

    // The Static Lifetime

    /*
    One special lifetime worth discussing is the static lifetime, which denotes
    that the affected reference can live for the entire duration of a program.
    All string literals have the static lifetime, which could be annotated as:
    */
    let s: &'static str = "I have a static lifetime.";
    println!("{s}");

    /*
    The text of this string is stored directly in the programs binary, which
    is always available, and so has a static lifetime
    Sometimes compilation errors will suggest specifying a static lifetime,
    but most times this is due to some dangling reference errors which should be fixed
    */

    // Generic Type Parameters, Trait Bounds, and Lifetimes Together
    // Here is what this might look like all together in a single function

    fn _longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where
        T: Display,
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}
