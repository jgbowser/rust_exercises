// 8.2 Storing UTF-8 Encoded Text with Strings

// Strings tend to present more issues than most new Rust programmers would expect
// there are 3 reasons for this:
// 1. Rust's propensity for exposing possible errors
// 2. Strings being a more complicated data structure than they are given credit for
// 3. UTF-8

// strings are implemented as a collection of bytes, plus some methods to provide
// useful functionality when the bytes are interpreted as text.
pub fn run() {
    /*
    What is a String?
    Rust only has 1 "string type" in the core, which is the "string slice" type: `str`,
    that is usually seen in its borrowed form : &str.
    string slices are references to some UTF-8 encoded string data stored else
    where. String literals for example are stored in the binary and therefore
    are string slices.

    The `String` type is provided by the standard library. It is growable,
    mutable, owned, and UTF-8 encoded.
    */

    // Creating a new String
    // String shares a lot of operations as Vec<T> because a String
    // is actually a wrapper around a vector of bytes. One example is `new()`
    let mut s = String::new();
    println!("what does just a new String look like? '{s}'");
    // we can load data into s now, some initial data to start the String with
    let data = "initial contents";
    s = data.to_string();
    println!("and now we have loaded data into s: '{s}'");
    // this works too: s = "initial contents".to_string();
    // We transform "initial contents" from a str to a String
    // this is the same as:
    let s = String::from("initial contents");
    println!("Another String from a str: '{s}'");
    // Strings are UTF-8, so any UTF-8 data work
    let _hello = String::from("السلام عليكم");
    let _hello = String::from("Dobrý den");
    let _hello = String::from("Hello");
    let _hello = String::from("שָׁלוֹם");
    let _hello = String::from("नमस्ते");
    let _hello = String::from("こんにちは");
    let _hello = String::from("안녕하세요");
    let _hello = String::from("你好");
    let _hello = String::from("Olá");
    let _hello = String::from("Здравствуйте");
    let _hello = String::from("Hola");

    // Updating a String
    // just like a vector, Strings can grow and change
    // There are a few methods of adding to a String

    // Appending to a String with push_str and push
    // push_str appends a string slice
    let mut string = String::from("foo");
    println!("our original String: {string}");
    string.push_str("bar");
    println!("Our String after push_str: {string}");
    // push_str takes a slice because we don't necessarily want to take ownership
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2); // <-- not taking ownership of s2
    println!("s2 is {}", s2); // <-- so we can use it later

    // the push method takes a single character (char) and adds it to a String
    let mut s3 = String::from("lo");
    s3.push('l');
    println!("pushed char: 'l' on the end: {s3}");

    // Concatenation with the + Operator or format! Macro
    {
        let s1 = String::from("Hello");
        let s2 = String::from(", world!");
        println!("s1 is: {s1}");
        println!("s2 is: {s2}");
        let s3 = s1 + &s2; // s1 is moved to here and can't be used again, s2 can;
        println!(
            "s1 + &s2 = {s3}. Also, s2 is still available because it was a reference, s2: {s2}"
        );
    }
    // the `+` operator uses the `add` method whose signature looks like:
    // fn add(self, s: &str) -> String {}
    // we can only add a reference to the first string, we can't add 2 Strings together
    // how does this work if s2: String? that means we're passing &String, not &str
    // this works because the compiler coerces &String to &str. Essentially, it
    // turns &s2 into &s2[..] where we "spread" our String into a slice. This is
    // called deref coercion (Chapter 15)

    // For more complicated string combinations we use format! macro because lots
    // of +'s gets unwieldy very quickly.

    {
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        let s = format!("{}-{}-{}", s1, s2, s3);
        println!("Our three strings formatted together is: {s}");
    }
    // format! works a lot like println! except instead of printing the output on
    // the screen it returns a String with the contents. This call also doesn't take
    // ownership of any of the arguments.

    // Indexing into Strings
    /*
    this fails to compile:
    let s1 = String::from("hello");
    let h = s1[0];
    */
    // Rust strings don't support indexing. This is due to how Rust stores
    // strings in memory

    // String Storage: Internal Representation
    /*
    A String is a wrapper over a Vec<u8>
    examples:
    let hello = String::from("hola");
    in this case the len is 4, which means the vector storing the string "hola"
    is 4 bytes long. each letter takes 1 byte when UTF-8 encoded. 
    This example may be surprising though..
    let hello = String::from("Здравствуйте");
    you might think the len here is 12...actually it is 24. This is because these
    characters each take 2 bytes in UTF-8
    so.. if we tried to index into this string, say &hello[0] it wouldn't actually
    corelate with the expected first character: "З" (Cyrillic Ze, not 3)
    */

    // Bytes and Scalar Values and Grapheme Clusters! Oh My!
    /* 
    Another point about UTF-8 is there are actually 3 ways to look at Strings
    from Rust's perspective: as bytes, scalar values, and grapheme clusters (the closest thing to "letters")

    If we look at the Hindi word “नमस्ते”, it is stored as a vector of
    u8 values that looks like this:
    [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
    that's 18 bytes...
    If we look at them  as Unicode scalar values (Rust's `char` type) those bytes look like:
    ['न', 'म', 'स', '्', 'त', 'े']
    there are 6 `char` values here, but the 4th and 6th aren't letters, they are
    diacritics that don't make sense on their own.
    as grapheme clusters we'd get what you would call 4 letters that make up
    the hindi word, and they look like:
    ["न", "म", "स्", "ते"]
    */

    // Slicing Strings
    // if we do want to "index into" a string, Rust requires us to be a bit more specific
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("our slice from [0..4] is equivalent to: {s}");
    // this is a slice of 4 bytes, 2 bytes per character = 2 characters.
    // If this wasn't a valid slice, eg [0..1] Rust would panic at runtime
    // this operation should be done with caution to avoid program crashes

    // Methods for Iterating Over Strings
    // the best way to operate on pieces of strings is to be explicit about
    // whether you want characters or bytes. For individual Unicode scalar values,
    // use the `chars` method
    for c in "Зд".chars() {
        println!("Iterating over chars: {c}");
    }
    // or we can iterate over raw bytes:
    for b in "Зд".bytes() {
        println!("Iterating over the same as bytes: {b}");
    }

    /*
    Summary: Strings are not so simple in Rust.
    This is because most languages abstract away the complexities of strings,
    but allow for many more potential bugs. Rust chooses to surface more of the
    complexity to the programmer, meaning they have to put more thought into
    handling them, but in doing so, helps avoid many bugs
    */
}
