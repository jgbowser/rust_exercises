// 4.3 The Slice Type
// Slice behaves much the same as it does in other languages in that it allows
// you to reference a contiguous sequence of elements in a collection
// In Rust, a slice is a kind of reference so it doesn't have ownership

pub fn run() {
    /*
    Here’s a small programming problem: write a function that takes a
    string of words separated by spaces and returns the first word it finds
    in that string. If the function doesn’t find a space in the string, the
    whole string must be one word, so the entire string should be returned.
    */

    fn _first_word(s: &String) -> usize { // take in a string reference and return the index of the end of the first word, indicated by a space
        let bytes = s.as_bytes(); // to step over the String element by element, break it up into an array of bytes
        for (i, &item) in bytes.iter().enumerate() { //create an iterator and enumerate (gives a tuple with the index and a ref to the value) and loop
            if item == b' ' { //this is byte literal syntax, we are matching the byte that is a space
                return i; //if we found a space, return the position
            }
        }
        s.len() // no space was found, this is one word, return the length
    }

    // first_word() presents a problem. The result we return is only meaningful
    // in the context of the original String. If that value changes or gets dropped
    // the result is meaningless. We can't guarantee it is a valid value in the future
    // what if we were trying to find the second word, now we need a start and end index to keep track off...
    // We need a way to keep all this data in sync...
    //... String Slices

    let s = String::from("hello world");
    // let hello = &s[0..5]; //ending index is non-inclusive
    let hello = &s[..5]; //can drop the zero, this is equivalent
    // let world = &s[6..11];
    let world = &s[6..]; // can drop the last index, these are also equivalent
    // dropping both values [..] = a slice of the whole string
    println!("this is the '{hello}' slice [0..5]. And this is the '{world}' slice [6..11].");

    // let's rewrite first_word now that we know how to use slices
    fn _first_word_slice(s: &String) -> &str { // returns &str, the type of a String slice
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        &s[..]
    }
    // to make this function more useful generally, we can change the signature:
    // fn first_word(s: &str) -> &str
    // this allows us to pass in string literals, and also Strings, by first slicing:
    /*
    let hello = "hello";
    let string = String::from("hello");
    first_word(hello);
    first_word(&string[..3]);
    first_word(&string);
    etc...
    */

    // it doesn't stop at string slices, what about arrays?
    let a = [1, 2, 3, 4, 5];
    let slice = &a[0..3];
    assert_eq!(slice, &[1, 2, 3]);
    println!("[0..3] slice of an array: {:?}", slice);
}
