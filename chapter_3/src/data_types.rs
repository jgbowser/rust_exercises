use std::io;
// 3.2 Data Types

pub fn _run() {
    // Scalar Type: represents a single value.
    // 4 primary types: integers, floating-points, booleans, characters

    /*
      Integer Types
      LENGTH    SIGNED  UNSIGNED
      8-bit	    i8	    u8
      16-bit	  i16	    u16
      32-bit	  i32	    u32
      64-bit	  i64	    u64
      128-bit	  i128	  u128
      arch	    isize	  usize


      Number literals	 Example
      Decimal	         98_222
      Hex	             0xff
      Octal	           0o77
      Binary	         0b1111_0000
      Byte (u8 only)	 b'A'
    */

    // Floating Point
    // f32 & f64
    let _x = 2.0; // f64
    let _y: f32 = 3.0; //f32

    // Numeric Operations
    // addition
    let sum = 5 + 10;
    println!("sum of 5 and 10 is {sum}");

    // subtraction
    let difference = 95.5 - 4.3;
    println!("difference of 95.5 and 4.3 is {difference}");

    // multiplication
    let product = 4 * 30;
    println!("product of 4 and 30 is {product}");

    // division
    let quotient = 56.7 / 32.2;
    println!("result of 56.7 / 32.2 is {quotient}");
    let floored = 2 / 3; // Results in 0
    println!("integer division rounds down to nearest integer. 2 / 3 = {floored}");

    // remainder
    let remainder = 43 % 5;
    println!("43 % 5 has a remainder of {remainder}");

    // Booleans
    let _t = true;
    let _f: bool = false;

    // Character Type
    // '' denotes a char type. double quotes denote strings
    // can be any single unicode character. This includes emoji and other special chars
    let _c = 'z';
    let _z: char = 'ℤ'; // with explicit type annotation
    let _heart_eyed_cat = '😻';

    // Compound Types
    // group multiple values into one type
    // 2 compound types: Tuples and Arrays

    // Tuple
    // group together a number of values with a variety of types. Fixed size
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // destructuring a tuple
    let (x, y, z) = tup;
    println!(
        "the value of x is: {}. the value of y is: {}. the value of z is: {}",
        x, y, z
    );

    // accessing a tuple value directly using index and dot notation
    let x = (500, 6.4, 1);
    println!("access the first index of tuple: x.0 = {}", x.0);

    // Array
    // all elements must have same type. Fixed length
    // allocated on stack rather than heap (more on this in later chapters)
    // Vectors are more common and are basically non-fixed length arrays
    //      [type; length]
    let _a: [i32; 5] = [1, 2, 3, 4, 5];
    // initialize with same value for each element
    let _b = [3; 5]; // ==> [3, 3, 3, 3, 3]
                     // accessing array elements
    let arr = [1, 2, 3, 4, 5];

    let first = arr[0];
    let second = arr[1];
    println!(
        "access array elements with bracket notation: a[0] = {} and a[1] = {}",
        first, second
    );

    // Accessing invalid index
    // unlike many other lower-level languages Rust protects you from
    // accessing non-existent indexes of an array and potentially undesired
    // parts of memory

    let array = [1, 2, 3, 4, 5];
    println!("Please enter an index. (indexes > 4 will cause a panic...)");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    let index: usize = index.trim().parse().expect("Index entered was not a number");
    let element = array[index];
    println!("the value of the element at index {index} is: {element}");
}
