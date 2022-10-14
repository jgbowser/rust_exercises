// Chapter 15: Smart Pointers

/*
A pointer is a general concept for a variable that contains an address in memory.
This address refers to, or "points at," some other data. The most common kind of
pointer in Rust is a reference. Smart pointers are data structures that act like
a pointer but also have additional metadata and capabilities. The concept of
smart pointers isn't unique to Rust: smart pointers originated in C++ and exist
in other languages as well.

An additional difference between references and smart pointers is that, in many
cases, smart pointers own the data they point to, rather than borrow it.

Smart pointers are usually implemented using structs. Unlike an ordinary struct,
smart pointers implement the Deref and Drop traits.

The Deref trait allows an instance of the smart pointer struct to behave like a
reference so you can write your code to work with either references or smart pointers.

The Drop trait allows you to customize the code that's run when an instance of
the smart pointer goes out of scope.

We'll be covering the most common smart pointers:
    Box<T>: for allocating values on the heap
    Rc<T>: a reference counting type that enables multiple ownership
    Ref<T> and RefMut<T>: accessed through RefCell<T>, a type that enforces the
    borrowing rules at runtime instead of compile time.
*/

mod box_pointer;
mod deref_trait;
mod drop_trait;

fn main() {
    // box_pointer::run();
    // deref_trait::run();
    drop_trait::run();
}
