// 10.2 Traits: Defining Shared Behavior

// A Trait defines functionality a particular type has, and can share with other types
// we can use Traits to further constrain generic types to types that have certain behaviors

pub fn _run() {
    // Defining a Trait
    // A type's behavior consists of the methods we can call on that type.
    // Different types share the same behavior if we can call the same methods
    // on those types. Trait definitions are a way to group method signatures
    // together to define a set of behaviors necessary to accomplish some purpose.

    // An example
    // Say we have multiple structs that hold various kinds and amounts of text:
    // a NewsArticle struct that holds a news story filed in a particular location
    // a Tweet that can have 280 characters along with metadata that indicates whether it was a new tweet, retweet, or reply

    // We want to make a media aggregator library crate named `aggregator` that can
    // display summaries of data that might be stored in a NewsArticle or Tweet instance
    // To do this, we need a summary for each type, and we'll get that via a
    // `summarize` method on an instance. Let's setup a Summary Trait
    /*
    pub trait Summary {
        fn summarize(&self) -> String;
    }
    */

    // we define the method signature, but not the implementation details
    // each type that implements this Trait must provide it's own behavior
    // for the body of the method. The compiler will enforce that any type
    // that has the Summary Trait will have the method summarize defined with
    // this signature exactly. A Trait can have multiple methods in its body:
    // the method signatures are listed one per line and each ends in a semicolon.

    // Implementing a Trait on a Type

    // Now that we've defined the signatures of our Summary trait's methods,
    // we can implement it on the types in our media aggregator.
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    // impl Summary for NewsArticle {
    //     fn summarize(&self) -> String {
    //         format!("{}, by {} ({})", self.headline, self.author, self.location)
    //     }
    // }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    // impl Summary for Tweet {
    //     fn summarize(&self) -> String {
    //         format!("{}: {}", self.username, self.content)
    //     }
    // }

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    // Default Implementations
    // Instead of requiring every type to implement custom behavior themselves
    // sometimes it is nice to implement default behavior if custom behavior isn't
    // defined for a type. We'll comment out the Summary definition above, and redo
    // it here with some default behavior

    pub trait Summary {
        fn summarize(&self) -> String {
            String::from("Read more...")
        }
    }

    impl Summary for NewsArticle {}

    // now we setup an instance of NewsArticle that uses the default behavior
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());

    // Traits can also be defined with default behavior in one method that
    // calls another method on the trait that requires custom behavior
    // commenting Summary above for another example

    pub trait Summary2 {
        fn summarize_author(&self) -> String;
        fn summarize(&self) -> String {
            format!("Read more from {}...", self.summarize_author())
        }
    }

    // to use this version of Summary, we only need to define summarize_author
    // when we implement the trait

    impl Summary2 for Tweet {
        fn summarize_author(&self) -> String {
            format!("@{}", self.username)
        }
    }

    println!("{}...{}", tweet.content, tweet.summarize());

    // Traits as Parameters
    // Using traits we can define functions that accept many different types,
    // as long as those types implement a specific trait.

    // pub fn notify(item: &impl Summary2) {
    //     println!("Breaking news! {}", item.summarize());
    // }

    // the impl syntax means any thing the implements the Summary2 trait may
    // be passed into the function. This is actually just syntactic sugar for

    // pub fn notify<T: Summary2>(item: &T) {
    //     println!("Breaking news! {}", item.summarize());
    // }

    // this is called "trait bound" syntax. This version allows for the defining
    // of functions with more complex cases.
    // for example if our function takes 2 parameters, and both must impl Summary2:

    // pub fn notify(item1: &impl Summary2, item2: &impl Summary2) {}

    // this definition is good if we want to allow item1 and item2 to have 2 different
    // types, that both implement Summary2.
    // If we want to force them both to have the same type though, we need to use
    // the trait bound style

    // pub fn notify<T: Summary2>(item1: &T, items2: &T) {}

    // the generic type 'T' applied to both parameters means they both must be the same

    // Specifying Multiple Trait Bounds with the '+' Syntax
    // This syntax is used when we want a parameter to implement multiple  traits

    // pub fn notify(item: &(impl Summary + Display)) {}

    // it also works with trait bound style

    // pub fn notify<T: Summary + Display>(item: &T) {}

    // Clearer Trait Bounds with `where` Clauses
    // Using Trait Bounds can get pretty messy when multiple generic types
    // are defined for a function as they all need their own trait bounds.
    use std::fmt::{Debug, Display};
    fn _some_function<T: Display + Clone, U: Clone + Debug>(_t: &T, _u: &U) {}
    // there is a ton of information between the function name and parameters that
    // end up making this pretty difficult to read. For this reason Rust lets us
    // use the `where` clause to improve the readability

    fn _more_readable<T, U>(_t: &T, _u: &U) -> i32
    where
        T: Display + Clone,
        U: Clone + Debug,
    {
        42
    }

    // Returning Types that Implement Traits
    // We can also use the `impl Trait` syntax in the return position to return
    // a value of some type that implements a trait
    fn _returns_summarizable() -> impl Summary2 {
        Tweet {
            username: String::from("Horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    }

    // We don't name a concrete type here, but we return "something" that implements
    // The Summary2 type
    // This behavior is especially useful in the context of closures and iterators (Ch 13)
    // those end up often being very long to specify, or something only the compiler
    // knows, so being able to return some type that implements the Iterator trait
    // is much more concise
    // this only works if a single type is returned from the function
    // the following function won't compile

    /*
    fn _returns_summarizable2(switch: bool) -> impl Summary {
        if switch {
            NewsArticle {
                headline: String::from("Penguins win the Stanley Cup Championship!"),
                location: String::from("Pittsburgh, PA, USA"),
                author: String::from("Iceburgh"),
                content: String::from(
                    "The Pittsburgh Penguins once again are the best \
                     hockey team in the NHL.",
                ),
            }
        } else {
            Tweet {
                username: String::from("horse_ebooks"),
                content: String::from("of course, as you probably already know, people"),
                reply: false,
                retweet: false,
            }
        }
    }
    */

    // Using Trait Bounds to Conditionally Implement Methods
    // By using a trait bound with an impl block that uses generic type parameters,
    // we can implement methods conditionally for types that implement the specified traits.

    struct Pair<T> {
        x: T,
        y: T
    }

    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }

    // We can also conditionally implement a trait for any type that implements
    // another trait. Implementations of a trait on any type that satisfies the
    // trait bounds are called blanket implementations and are extensively used
    // in the Rust standard library. For example, the standard library implements
    // the ToString trait on any type that implements the Display trait. The impl
    // block in the standard library looks similar to this code:

    // impl<T: Display> ToString for T {}
    // let s = 3.to_string();
}
