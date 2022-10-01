// 13.2 Processing a Series of Items with Iterators

pub fn run() {
    /*
    The iterator pattern allows us to perform some task on a sequence of items in turn.
    An iterator is responsible for the logic of iterating over each item and
    determining the sequence has finished, meaning we don't have to implement that
    logic ourselves when using iterators.

    In Rust iterators are lazy, meaning they have no effect until you call the methods
    that consume the iterator to use it up. The following code creates, but doesn't
    actually use the iterator
    */
    let v1 = vec![1, 2, 3];
    let _v1_iter = v1.iter();

    /*
    Continuing this pattern though, we can create the iterator, then loop over it
    using a for loop
    */
    let v2 = vec![1, 2, 3];
    let v2_iter = v2.iter();

    for val in v2_iter {
        println!("{val}");
    }

    // The Iterator Trait and next Method

    /*
    All iterators implement a trait named Iterator that is defined in the std.
    the definition looks like:

    pub trait Iterator {
        type Item;
        fn next(&mut self) -> Option<Self::Item>;
    }

    this definition uses some new syntax. `type Item` and `Self::Item`, which are
    defining an associated type with this trait (Chapter 19). Basically, implementing
    the Iterator trait means you also need to define an Item type, which must be used
    in the return of the next method. The Item type is what gets returned from the iterator.

    We can even call the next method directly (see the test block below)

    There are a couple flavors of iterators:
    1. iter -> values we get are immutable references
    2. into_iter -> values we get are owned values
    3. iter_mut -> values we get are mutable references.
    */

    // Methods that Consume the Iterator

    /*
    The Iterator trait has a number of different methods with the default implementations
    provided by the standard library. Some of the methods call next() in their definition,
    which is why we are required to implement next() when implementing the Iterator trait.

    Methods that call next() are called consuming adaptors, because calling them uses
    up the iterator. One example is the sum method, which takes ownership of the iterator
    and iterates through the items, repeatedly calling next, using up the iterator.
    (example in test block below)
    */

    // Methods that Produce Other Iterators

    /*
    Iterator adaptors are method defined on the Iterator trait that don't consume the
    iterator. Instead, they produce different iterators by changing some aspect of the original.
    An example of this is the map() method
    */

    let v3 = vec![1, 2, 3];
    // v1.iter().map(|x| x + 1); this results in an error, we aren't doing anything with the new iterator
    // we can fix the error above by collecting the iterator into a new variable
    let v4: Vec<_> = v3.iter().map(|x| x + 1).collect();

    assert_eq!(v4, vec![2, 3, 4]);

    // Using Closures that Capture Their Environment
    /*
    Many iterator adaptors take closures as their arguments, and commonly the closures
    we'll specify as arguments to iterator adaptors will be closures tht capture their
    environment.

    For this example we use the filter() method that takes a closure. The closure looks
    at the current item and returns a bool based on the closure.

    In the following example we use a closure that captures the shoe_size variable from
    it's environment to iterate over a collection of Shoe struct instances and returns shoes
    of the specified size.
    */

    #[derive(Debug, PartialEq)]
    pub struct Shoe {
        size: u32,
        style: String,
    }

    pub fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        shoes.into_iter().filter(|s| s.size == shoe_size).collect()
    }

    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_in_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe {
                size: 10,
                style: String::from("sneaker")
            },
            Shoe {
                size: 10,
                style: String::from("boot")
            },
        ]
    );
}
// Calling next() ourselves
#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();
    // We make this mut, because calling next() changes the internal state of the iterator.
    // We don't do this explicitly with a for loop, but the loop does it behind the scenes

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

// Sum consumes the iterator
#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}
