// 10.2 Traits: Defining Shared Behavior

// A Trait defines functionality a particular type has, and can share with other types
// we can use Traits to further constrain generic types to types that have certain behaviors

pub fn run() {
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

    impl Summary for NewsArticle {

    }

    // now we setup an instance of NewsArticle that uses the default behavior
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
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
}
