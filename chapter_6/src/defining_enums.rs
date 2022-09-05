// 6.1 Defining Enums
// a way of saying a value is one of a possible set of values

pub fn run() {
    // an example using IP Address types (4 or 6)
    enum IpAddrKind { // notice the lack of snake case here
        V4,
        V6
    }

    // Creating instances of each of the variants
    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;
    // '::' indicates: Namespaced. So both 'four' and 'six' are of type 'IpAddrKind', but are different variants
    fn _route(_ip_kind: IpAddrKind) {} // both V4 or V6 variants are accepted here
    
    // right now we aren't storing any data about the IP Address, we only know
    // what kind it is. We could tackle storing the data using structs..
    struct IpAddrStruct {
        _kind: IpAddrKind,
        _address: String,
    }

    let _home = IpAddrStruct {
        _kind: IpAddrKind::V4,
        _address: String::from("127.0.0.1"),
    };

    let _loopback = IpAddrStruct {
        _kind: IpAddrKind::V6,
        _address: String::from("::1"),
    };
    // this looks ok, however, we can represent the same concept, more concisely,
    // using just an enum. Rather than an enum inside a struct, we can put the data
    // directly into each enum variant. This new definition lets us know that each
    // variant will have associated String values
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let _home2 = IpAddr::V4(String::from("127.0.0.1"));
    let _loopback2 = IpAddr::V6(String::from("::1"));
    // This makes it clear that the name of each enum variant is also a function
    // that constructs an instance of the enum. IpAddr::V4(s: String) -> IpAddr.

    // Another advantage of using an enum rather than a struct is the ability to
    // give each variant unique types
    enum IpAddr2 {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let _home3 = IpAddr2::V4(127, 0, 0, 1);
    let _loopback3 = IpAddr2::V6(String::from("::1"));

    // The example we have been using is actually pretty common. So common, in fact,
    // that the standard library already implements this enum. let's look at how
    // the standard library implements this.
    /*
    struct Ipv4Addr {
        // --snip--
    }

    struct Ipv6Addr {
    // --snip--
    }

    enum IpAddr {
        V4(Ipv4Addr),
        V6(Ipv6Addr),
    }

    it defines each IP type as a struct, then uses that as the type
    for each variant constructor
    */

    // another example with a variety of types
    enum Message {
        _Quit,
        _Move { x: i32, y: i32 },
        Write(String),
        _ChangeColor(i32, i32, i32),
    }
    /*
    This enum has four variants with different types:
        Quit has no data associated with it at all.
        Move has named fields like a struct does.
        Write includes a single String.
        ChangeColor includes three i32 values.
    this is equivalent to:
        struct QuitMessage; // unit struct
    struct MoveMessage {
        x: i32,
        y: i32,
    }
    struct WriteMessage(String); // tuple struct
    struct ChangeColorMessage(i32, i32, i32); // tuple struct

    but with the enum we have the benefit of all this structure being contained
    under a single type.
    */

    // We can also define methods on enums
    impl Message {
        fn call(&self) {
            // method body
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();

    // The Option Enum and Its Advantages Over Null Values
    // Option: could be something, or it could be nothing
    // Rust doesn't have `null`, but it has an enum that represent the concept of null
    enum _Option<T> {
        None,
        Some(T),
    }
    // this is included in the prelude (you don't need to manually bring it into scope).
    // the <T> syntax is a generic type parameter (chapter 10).
    // when a value has an Option<T> type we the compiler can't be confident that
    // we have a valid value. We need to do some null checks and correctly handle
    // that possibility before we perform operations using that value. In other words,
    // Option<T> --> T before we do work. 
    // the Option enum has a lot of methods that make it easy to ensure we have a
    // valid value.
}