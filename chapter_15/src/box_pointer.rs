// 15.1 Using Box<T> to Point to Data on the Heap

/*
Box is the most straightforward smart pointer, written as Box<T>.
Boxes allow us to store data on the heap rather than the stack . What remains
on the stack is the pointer to the heap data. Boxes don't have performance overhead
other than storing in the heap rather than the stack. Used mostly in the following
situations:
 - When you have a type whose size can't be known at compile time and you want
    to use a value of that type in a context that requires an exact size
 - When you have a large amount of data and you want to transfer ownership but
    ensure the data won't be copied when you do so.
 - When you want to own a value and you care only that it's a type that implements
    a particular trait rather than being a specific type
*/
pub fn run() {
    // Using a Box<T> to Store Data on the Heap

    // but first, Box syntax and interaction
    let b = Box::new(5);
    println!("b = {}", b);

    // variable 'b' has a value of a 'Box' that points to the value '5' on the heap
    // this isn't a very realistic situation though, we rarely want single values on the heap

    // Enabling Recursive Types with Boxes

    /*
    A value of recursive type can have another value of the same type as part of itself.
    This normally is an issue because Rust needs to know how much space a type
    takes up at compile time. Because boxes have a known size, we can enable recursive
    types by inserting a  box in the recursive type definition.

    To explore this we'll use the 'cons list' recursive type as an example. apparently,
    this is a common data structure/type.
    */

    // More information about the Cons List

    /*
    A cons list is a data structure that comes from Lisp and is made up of nested pairs
    and is basically the Lisp version of a Linked List.

    psuedocode representation of a list containing 1, 2, 3 with each pair in parens
    (1, (2, (3, Nil)))

    each item in a cons list contains two elements: the value of the current item
    and the next item (Nil is the end of the list).

    This really isn't a common pattern in Rust, but it is a fairly straightforward
    example that will illustrate the point

    below is kind of how the code might look, but we haven't used Box yet so
    it fails to compile, we'll dig into how Box works below..I think
    */

    /*
    use List::{Cons, Nil};

    enum List {
        Cons(i32, List),
        Nil,
    }

    let list = Cons(1, Cons(2, Cons(3, Nil)));
    */

    // Computing the Size of a Non-Recursive Type

    // Normally Rust looks at the variant of an enum that requires the most space to
    // store when determining how to allocate memory

    // Using Box<T> to Get a Recursive Type with a Known Size

    /*
    Because Rust can't figure out how much space to allocate with recursive types
    it will fail to compile and provide a useful suggestion.

    Because Box<T> is a pointer, Rust already knows how much space it needs, a 
    pointer's size doesn't change based on the amount of data it points to.
    We can now change the code above to look like this:
    */
    
    enum List {
        Cons(i32,  Box<List>),
        Nil,
    }
    
    use List::{Cons, Nil};

    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
