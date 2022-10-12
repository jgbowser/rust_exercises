// 15.2 Treating Smart Pointers Like Regular References with the Deref Trait

/*
Implementing the Deref trait allows you to customize the behavior of the 
dereference operator * (not multiplication or glob). By implementing Deref in
such a way that a smart pointer can be treated like a regular reference, you can
write code that operates in references and use that code with smart pointers too.

First we'll look at how the dereference operator works with regular references.
Then we'll try to define a custom type that behaves like Box<T>, and see why the
dereference operator doesn't work like a reference on our newly defined type.
We'll explore how implementing the Deref trait makes it possible for smart pointers
to work in ways similar to references. Then we'll look at Rust's deref coercion
feature and how it lets us work with ref or smart pointers.
*/

pub fn run() {
    // Following the Pointer to the Value

    /*
    A regular reference is a type of pointer, and one way to think of a pointer
    is as an arrow to a value stored somewhere else. in the following example we
    create a reference to an i32 value and then use the dereference operator to 
    follow the reference to the value
    */

    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y); // can't compare a value to a pointer, so dereference the ref

    // Using Box<T> Like a Reference

    /*
    the example above could be rewritten using a Box and behave in the same way
    */

    let x2 = 5;
    let y2 = Box::new(x2);

    assert_eq!(5, x2);
    assert_eq!(5, *y2);

    /*
    The main difference between these two examples is that  in the second one, rather
    than setting y2 to a reference pointing to the value of x2 we set y2 to be an
    instance of a box pointing to a copied value of x.
    */

    // Defining Our Own Smart Pointer

    /*
    Lets build a smart pointer similar to Box<T> to experience how smart pointers
    behave differently from references by default. Then we'll look at how to add 
    the ability to use the dereference operator. 

    The Box<T> type is ultimately defined as a tuple struct with one element, so
    we will define MyBox the same way. We'll also define a new function like on
    the Rust version.
    */

    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    /*
    At this point if we try to substitute MyBox for Box in the example above we
    will get a compilation error, because our MyBox type doesn't implement the
    Deref trait.
    */

    // Treating a Type Like a Reference by Implementing the Deref Trait

    /*
    To implement a trait (like Deref) we need to provide implementations for the
    trait's required methods. The Deref trait requires us to implement the deref
    method that borrows self and returns a reference to the inner data.
    */

    use std::ops::Deref;

    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    /*
    The 'type Target = T' syntax defines an associated type for the Deref trait to
    use. Associated types are a slightly different way of declaring a generic param,
    but we don't need to worry about them for now (Chapter 19).

    We fill the body of the deref method with '&self.0' so deref returns a reference
    to the value we want to access with the * operator. 
    */

    let x3 = 5;
    let y3 = MyBox::new(x3);

    assert_eq!(5, x3);
    assert_eq!(5, *y3);

    // Implicit Deref Coercions with Functions and Methods

    /*
    Deref coercion converts a reference to a type that implements the Deref trait
    into a reference to another type. For example, deref coercion can convert &String
    to &str because String implements  the Deref trait such that it returns &str.
    Deref coercion is a convenience Rust performs on arguments to functions and methods,
    and works only on types that implement the Deref trait. It happens automatically 
    when we pass a reference to a particular type's value as an argument to a function
    or method that doesn't match the parameter type in the function or method definition.

    Deref coercion was added to Rust so that programmers writing function and method
    calls don't need to add as many explicit references and dereferences with & and *.
    It also allows us to write more code that can work for either references or smart pointers.

    let see an example using the MyBox type we made
    */

    fn hello(name: &str) {
        println!("Hello, {name}");
    }

    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    // without Rust deref coercing this for us it would be much more complicated looking

    /*
    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);

    The (*m) dereferences the MyBox<String> into a String. Then the & and [..] take
    a string slice of the String that is equal to the whole string to match the
    signature of hello. This code without deref coercions is harder to read, write,
    and understand with all of these symbols involved. Deref coercion allows Rust to
    handle these conversions for us automatically.
    */

    // How Deref Coercion Interacts with Mutability

    /* 
    DerefMut is the mutable reference trait version of the Deref trait
    
    Rust does deref coercion when it finds types and trait implementations in 3 cases:
    1. From &T to &U when T: Deref<Target=U>
    2. From &mut T to &mut U when T: DerefMut<Target=U>
    3. From &mut T to &U when t: Deref<Target=U>

    The first case states that if you have a &T, and T implements Deref to some type U,
    you can get a &U transparently. The second case states the same thing except for
    mutable references. 

    The third case is trickier: Rust will also coerce a mutable reference to an
    immutable one. But the reverse is not possible: immutable references will never
    coerce to mutable references. Because of the borrowing rules, if you have a
    mutable reference, that mutable reference must be the only reference to that
    data. Converting one mutable reference to one immutable reference will never
    break that rule, but going the other way very well could. 
    */
}