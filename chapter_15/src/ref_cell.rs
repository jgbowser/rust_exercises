// 15.5 RefCell<T> and the Interior Mutability Pattern

/*
Interior mutability is a design pattern in Rust that allows you to mutate data
even when there are immutable references to that data; normally, this action
is disallowed by the borrowing rules. To mutate data, the pattern uses unsafe
code inside a data structure to bend Rust's usual rules that govern mutation
and borrowing. Unsafe code indicates to the compiler that we're checking the
rules manually instead of relying on the compiler to check them for us.

We can use types that use the interior mutability pattern only when we can ensure
that the borrowing rules will be followed at runtime, even though the compiler can't
guarantee that. The unsafe code involved is then wrapped in a safe API, and the
outer type is still immutable.

We'll explore this concept by looking at the RefCell<T> type that follows the
interior mutability pattern.
*/
pub fn run() {
    // Enforcing Borrowing Rules at Runtime with RecCell<T>

    /*
    Unlike Rc<T>, the RefCell<T> type represents single ownership over the data
    it holds. Let's look at what makes RefCell<T> different from Box<T>.
    Recall the borrowing rules from Chapter 4:
    - At any time, you can have either(but not both) one mutable or any number
        of immutable references.
    - References must always be valid.

    With references and Box<T>, the borrowing rules' invariants are enforced at
    compile time. With RefCell<T>, these invariants are enforced at runtime. With
    references, if you break these rules, you'll get a compile error. With RefCell<T>,
    if you break these rules, your program will panic and exit.

    The advantage of checking the borrowing rules at runtime rather than compile
    time is that certain memory-safe scenarios are allowed, where they would've
    been disallowed by compile-time checks. There are scenarios that are impossible
    for the Rust compiler to perform accurate static analysis on. The RefCell<T>
    type is useful when you're sure your code follows the borrowing rules but
    the compiler is unable to understand or guarantee that.

    Similar to Rc<T>, RefCell<T> is only for use in single-threaded scenarios and
    will give compile-time errors if it is used in a multi-threaded context.

    A recap of reasons to choose Box<T>, Rc<T>, or RefCell<T>:
    - Rc<T> enables multiple owners of the same data, Box<T> and RecCell<T> have
        single owners.
    - Box<T> allows immutable or mutable borrows checked at compile time; Rc<T>
        allows only immutable borrows checked at compile time; RefCell<T> allows immutable
        or mutable borrows checked at runtime.
    - Because RefCell<T> allows mutable borrows checked at runtime, you can mutate
    the value inside the RefCell<T> even when the RefCell<T> is immutable.
    */

    // Interior Mutability: A Mutable Borrow to an Immutable Value

    /*
    A consequence of the borrowing rules is that when you have an immutable value,
    you can't borrow it mutably. The following won't compile:

    let x = 5;
    let y = &mut x;

    However, there are situations in which it would be useful for a value to mutate
    itself in its methods but appear immutable to other code. Code outside the value's
    Code outside the value's methods would not be able to mutate the value. Using
    RefCell<T> is one way to get the ability to have interior mutability, but
    RefCell<T> doesn't get around the borrowing rules completely: the borrow checker
    in the compiler allows this interior mutability, and the borrowing rules are
    checked at runtime instead. If we violate the rules we get a panic! instead of
    a compiler error.
    */

    // A Use Case for Interior Mutability: Mock Objects

    /*
    Sometimes during testing a programmer will use a type in place of another type,
    in order to observe particular behavior and assert it's implemented correctly.
    This placeholder type is called a test double. Think of it in the sense of a
    "stunt double." Mock objects are specific types of tst doubles that record what
    happens during a test so you can assert that the correct actions took place.

    Rust doesn't have objects in the same sense as other languages have objects, and
    Rust doesn't have mock object functionality built into the standard lib as
    some languages do. However, we can definitely create a struct that will serve
    the same purpose as a mock object.

    Here's the scenario we'll test: we'll create a library that tracks a value
    against a maximum value and sends messages based on how close to the maximum
    value the current value is. This library could be used to keep track of a users
    quota for the number of API calls they are allowed to make for instance.

    Our library will only provide the functionality of tracking how close to the
    maximum a value is and what the messages should be a what times. Users of the
    library will be expected to provide a mechanism for sending the messages. The
    library doesn't need to know that detail. All it needs is something that
    implements a trait we'll provide called Messenger.
    */
}

    pub trait Messenger {
        fn send(&self, msg: &str);
    }

    pub struct LimitTracker<'a, T: Messenger> {
        messenger: &'a T,
        value: usize,
        max: usize,
    }

    impl<'a, T> LimitTracker<'a, T>
    where
        T: Messenger,
    {
        pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
            LimitTracker {
                messenger,
                value: 0,
                max,
            }
        }

        pub fn set_value(&mut self, value: usize) {
            self.value = value;

            let percentage_of_max = self.value as f64 / self.max as f64;

            if percentage_of_max >= 1.0 {
                self.messenger.send("Error: You are over your quota!");
            } else if percentage_of_max >= 0.9 {
                self.messenger
                    .send("Urgent warning: You've used up over 90% of your quota!");
            } else if percentage_of_max >= 0.75 {
                self.messenger
                    .send("Warning: You've used up over 75% of your quota!")
            }
        }
    }

    /*
    One important part of this code is that the Messenger trait has one method called
    send that takes an immutable reference to self and the text of the message.
    This trait is the interface our mock object needs to implement so that the mock
    can be used in the same way a real object is. The other important part is that
    we want to test the behavior of the set_value method on the LimitTracker. We
    can change what we pass in for the value parameter, but set_value doesn't return
    anything for us to make assertions on. We want to be able to say that if we create
    a LimitTracker with something that implements the Messenger trait and a particular
    value for max, when we pass different numbers for value, the messenger is told
    to send the appropriate messages.

    We need a mock object that, instead of sending an email or text message when we
    call send, will only keep track of the messages it's told to send. We can create
    a new instance of the mock object, create a LimitTracker that uses the mock
    object, call the set_value method on LimitTracker, and then check that the mock
    object has the messages we expect.

    First an example that won't work:

    #[cfg(test)]
    mod tests {
        use super::*;

        struct MockMessenger {
            sent_messages: Vec<String>,
        }

        impl MockMessenger {
            fn new() -> MockMessenger {
                MockMessenger {
                    sent_messages: vec![],
                }
            }
        }

        impl Messenger for MockMessenger {
            fn send(&self, message: &str) {
                self.sent_messages.push(String::from(message));
            }
        }

        #[test]
        fn it_sends_an_over_75_percent_warning_message() {
            let mock_messenger = MockMessenger::new();
            let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

            limit_tracker.set_value(80);

            assert_eq!(mock_messenger.sent_messages.len(), 1);
        }
    }

    We can't modify the MockMessenger to keep track of the messages, because the
    send method takes an immutable reference to self. We also can't take the
    suggestion from the error text to use &mut self instead, because then the
    signature of send wouldn't match the signature in the Messenger trait definition.

    This is a situation in which interior mutability can help! We'll store the sent_messages
    within a RefCell<T>, and then the send method will be able to modify sent_messages
    to store the messages we've seen. 
    */

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message))
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);   
    }
}
