// 15.4 Rc<T>, the Reference Counted Smart Pointer

/*
In the majority of cases, ownership is clear: you know exactly which variable owns
a given value. However, there are cases when a single value might have multiple
owners. for example, in graph data structures, multiple edges might point to the
same node, and the node is conceptually owned by all of the edges that point to it.
A node shouldn't be cleaned up unless it doesn't have any edges pointing to it and
so has no owners.

You have to enable multiple ownership explicitly by using the Rust type Rc<T>,
which is an abbreviation for reference counting. The Rc<T> type keeps track of
the number of references to a value to determine whether or not the value is still
in use. If there are zero references to a value, the value can be cleaned up without
any references becoming invalid. 

We use the Rc<T> type when we want to allocate some data on the heap for multiple
parts of our program to read and we can't determine at compile time which part will
finish using the data last. If we knew which part would finish last, we could just
make that part the data's owner, and the normal ownership rules enforced at compile
time would take effect.

Note that Rc<T> is only for use in single-threaded scenarios.
*/

pub fn run() {
    // Using Rc<T> to Share Data

    /*
    To help illustrate the concept we'll go back to the cons list example, except
    this time we'll have 3 lists, where 2 of them point to the shared 3rd list.
    */

    // This won't compile, because the Cons own the data they hold so 'a' can't be moved
    // multiple times

    // enum List {
    //     Cons(i32, Box<List>),
    //     Nil,
    // }

    // use List::{Cons, Nil};


    // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a));

    // We could change the definition of Cons to fix this, but then we have to 
    // specify lifetime parameters and that may assume some scenarios related to
    // lifetimes that won't be true.
    // instead we'll change the definition to use Rc<T> instead of Box<T>

    use List::{Cons, Nil};
    use std::rc::Rc;

    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let _b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    let _c = Cons(4, Rc::clone(&a));
    println!("count after creating c = {}", Rc::strong_count(&a));
    {
        let _d = Cons(6, Rc::clone(&a));
        println!("count after creating d = {}", Rc::strong_count(&a));
    }
    println!("count after d goes out of scope = {}", Rc::strong_count(&a));

    // Cloning an Rc<T> Increases the Reference Count
    
    /*
    The println!s above illustrate the incrementing and decrementing of the 
    reference count as clones are created and go out of scope. 
    */
}