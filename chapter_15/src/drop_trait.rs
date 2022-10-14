// 15.3 Running Code on Cleanup with the Drop Trait

/*
Drop is the second trait important to smart pointers. It lets you customize
what happens whe a value is about to go out of scope. You can provide an
implementation for the Drop trait on any type,and that code can be used to release
resources like files or network connections. 

This trait is being introduced in the context of smart pointers because it is almost
always used when implementing a smart pointer. For example, when Box<T> is dropped
it will deallocate the space on the heap that the box points to.

In some languages, for some types, the programmer must call code to free memory or
resources every time they finish using an instance of those types. Examples include
file handles, sockets, or locks. If they forget, the system might become overloaded
and crash. In Rust, you can specify that a particular bit of code be run whenever
a value goes out of scope, and the compiler will insert this code automatically. 
As a result, we don't need to be as careful about placing cleanup code everywhere.
*/

pub fn run() {
    /*
    We specify the code to run when a value goes out of scope by implementing the
    Drop trait. The Drop trait requires that we implement one method named drop
    that takes a mutable reference to self. We'll implement drop with println! so
    we can see when drop gets called.
    */

    struct CustomSmartPointer {
        data: String,
    }

    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("Dropping CustomSmartPointer with data '{}'", self.data);
        }
    }

    let _c = CustomSmartPointer{
        data: String::from("my stuff")
    };
    let _d = CustomSmartPointer {
        data: String::from("other stuff")
    };
    println!{"CustomSmartPointers created."};

    // notice when we run this that variables are dropped in reverse order of creation

    // Dropping a Value Early with std::mem::drop

    /*
    Unfortunately, it's not straightforward to disable the automatic drop functionality.
    Disabling drop isn't usually necessary; the whole point of the Drop trait is that
    it's taken care of automatically. Occasionally, however, you might want to clean
    up a value early. One example is when using smart pointers that manage locks:
    you might want to force the drop method that releases the lock so that other 
    code in the same scope can acquire the lock. Rust doesn't let you call the Drop
    trait's drop method manually; instead you have to call the std::mem::drop function
    provided by the standard library if you want to force a value to be dropped
    before the end of its scope.
    */

    let e = CustomSmartPointer {
        data: String::from("More stuff"),
    };
    println!("CustomSmartPointer created... again.");
    drop(e);
    println!("CustomSmartPointer dropped before the end of run().");
}