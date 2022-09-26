// 13.1: Closures: Anonymous Functions that Capture Their Environment

pub fn run() {
    /*
    Rust's closures are anonymous functions you can save in a variable or pass
    as arguments to other functions. You can create the closure in one place and
    then call the closure elsewhere to evaluate it in a different context.
    Unlike other functions, closures can capture values from the cope in which
    they are defined. We'll demonstrate how these closure features allow for code
    reuse and behavior customization.
    */

    // Capturing the Environment with Closures

    /*
    First, we'll examine how we can use closures to capture values from the
    environment they're defined in for later use. Here's the scenario:
    Every so often, our t-shirt company gives away an exclusive, limited-edition
    shirt to someone on our mailing list as a promotion. People on the mailing list
    can optionally add their favorite color to their profile. If the person chosen
    for a free shirt has their favorite color, they get that color shirt, otherwise,
    they get whatever color there are the most shirts of.

    For this example, we'll use an enum called ShirtColo that has the variants
    Red and Blue. We represent the company's inventory with an Inventory struct
    that has the field named shirts that contains a Vec<ShirtColor> representing
    the shirt colors currently in stock. The method giveaway defined on Inventory
    gets the optional shirt color preference of the winner, and returns the color
    of shirt the person will get
    */

    #[derive(Debug, PartialEq, Copy, Clone)]
    enum ShirtColor {
        Red,
        Blue,
    }

    struct Inventory {
        shirts: Vec<ShirtColor>,
    }

    impl Inventory {
        fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
            user_preference.unwrap_or_else(|| self.most_stocked())
        }
        /*
        unwrap_or_else takes a closure, the closure is `|| self.most_stocked()`. If there is Some(user_preference), unwrap_or_else returns
        that value, otherwise it returns the closure computed value (the most stocked shirt color in this case). If the closure took an argument
        it would go between the pipe characters.
        */

        fn most_stocked(&self) -> ShirtColor {
            let mut num_red = 0;
            let mut num_blue = 0;

            for color in &self.shirts {
                match color {
                    ShirtColor::Red => num_red += 1,
                    ShirtColor::Blue => num_blue += 1,
                }
            }
            if num_red > num_blue {
                ShirtColor::Red
            } else {
                ShirtColor::Blue
            }
        }
    }

    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    // Closure Type Inference and Annotation

    /*
    There are several differences between closures and normal functions. One of those is that closures
    don't usually require us to annotate types for the parameters or the return value. Types are
    required on functions because they are part of an explicit interface that is exposed to users
    of the code, closures are not exposed like this so the type annotations are not required.

    if we did want to annotate types, and in this case, store the closure in a variable,
    it would look like this:

    let expensive_closure = |num: u32| -> u32 {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    Here are some some examples of a closure with different level of verbosity compared
    to a function that does the same thing:

    fn  add_one_v1   (x: u32) -> u32 { x + 1 }   <-- function
    let add_one_v2 = |x: u32| -> u32 { x + 1 };  <-- fully annotated closure
    let add_one_v3 = |x|             { x + 1 };  <-- closure without annotations
    let add_one_v4 = |x|               x + 1  ;  <-- remove optional brackets
    */

    // Capturing References of Moving Ownership

    /*
    Closures can capture values from their environment in the same three ways a function can take a parameter:
    1. borrow immutably
    2. borrow mutably
    3. taking ownership
    The closure decides which method to use automatically based on what the function does with the captured values

    This example captures an immutable reference to the vector named list because it only prints the value
    */

    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("from closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
    println!("");

    // in the next example we change the closure body so that it adds an element to the list vector,
    // the closure now captures a mutable reference

    let mut list2 = vec![1, 2, 3];
    println!("Before defining the closure: {:?}", list2);

    let mut borrows_mutably = || list2.push(7);

    // no print line here because when we define borrows_mutably it captures a mutable ref to list2
    // remember we can't have a mutable and immutable reference to the same thing in the same scope

    borrows_mutably();
    println!("after calling closure: {:?}", list2);
    println!("");

    // if you want to force the closure to take ownership even if it doesn't need to you
    // can use the `move` keyword before the parameter list.
    // This technique is mostly useful when passing a closure to a new thread to move
    // the data so it is owned by the new thread.

    use std::thread;

    let list3 = vec![1, 2, 3];
    println!("Before defining the closure: {:?}", list3);

    thread::spawn(move || println!("from thread: {:?}", list3))
        .join()
        .unwrap();

    // Moving Captured Values Out of Closures and the `Fn` Traits

    /*
    Once a closure has captured a reference or ownership of a value from the environment where
    the closure is defined, the code in the body of the closure defines what happens to the
    references or values when the closure is evaluated later. A closure body can do any of
    the following: move a captured value out of the closure, mutate the captured value,
    do neither of these, or not capture anything to begin with.

    The way a closure captures and handles values from the environment affects which traits
    the closure implements, and traits are how functions and structs can specify what kinds
    of closures they can use. Closures will automatically implement one, two, or three of these
    `Fn` traits in an additive fashion, depending on how the closure's body handles the values

    1. FnOnce: applies to closures that can be called once. All closures implement at least 
        this trait, because all closures can be called. A closure that moves captured values
        out of its body will only implement FnOnce and none of the other Fn traits because
        it can only be called once. 
    2. FnMut: applies to closures that don't move captured values out of their body, but
        that might mutate the captured values. These closures can be called more than once.
    3. Fn: applies to closures that don't move captured values out of their bodies and that
        don't mutate captured values, as well as closures that capture nothing from their env.
        These closures can be called more than once without mutating their environment, which
        is important in cases such as calling a closure multiple times concurrently.

    let's look at the definition of the `unwrap_or_else` method on Option<T>

    impl<T> Option<T> {
        pub fn unwrap_or_else<F>(self, f: F) -> T 
        where
            F: FnOnce() -> T
        {
            match self {
                Some(x) => x,
                None => f(),
            }
        }
    }

    Remember that T is the generic type representing the type of the value in the Some variant
    and that the return type is also of type T.

    Notice the additional type parameter F. The F type is the type of parameter f, which is the
    closure we provide when calling unwrap_or_else.

    The trait bound specified on the F generic type is `FnOnce() -> T`, which means F must
    be able to be called once, take no arguments, and return a value of type T. Using FnOnce
    in the trait bound expresses the constraint that unwrap_or_else is only going to call f
    at most one time. In the body of unwrap_or_else, we can see that if the Option is Some,
    f won't be called. If the Option is None, f will be called once.

    Now let's look at the std method sort_by_key defined on slices to see how that differs from
    unwrap_or_else and why sort_by_key uses FnMut instead of FnOnce for the trait bound.
    The closure gets one argument in the form of a reference to the current item in the slice
    being considered, and returns a value of type K that can be ordered. This function is useful
    when you want to sort a slice by a particular attribute of each item.
    In the following example we sort a list of Rectangle instances by their width attribute; low -> high
    */

    # [derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let mut list = [
        Rectangle { width: 10, height: 1},
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    list.sort_by_key(|r| r.width); // this closure takes an FnMut trait function because it is called once for each item
    println!("{:#?}", list);
}
