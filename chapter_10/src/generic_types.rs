// 10.1 Generic Data Types

pub fn run() {
    // In function Definitions
    // we start with 2 functions tha find the largest value for different types
    fn largest_i32(list: &[i32]) -> &i32 {
        let mut largest = &list[0];
        for item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    fn largest_char(list: &[char]) -> &char {
        let mut largest = &list[0];
        for item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    let number_list = vec![34, 50, 25, 100, 65];
    let num_result = largest_i32(&number_list);
    println!("The largest number in the list is: {}", num_result);
    let char_list = vec!['y', 'm', 'a', 'q'];
    let char_result = largest_char(&char_list);
    println!("The largest char in the list is: {}", char_result);

    // both of these functions have the same logic, they just operate on
    // different types. We can fix this using generics
    fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];
        for item in list {
            if item > largest {
                // this ordering expression doesn't compile if we don't ensure T implements PartialOrd
                largest = item;
            }
        }
        largest
    }

    let result2 = largest(&number_list);
    println!("found the largest i32 using a generics func: {result2}");
    let result3 = largest(&char_list);
    println!("found the largest char using a generics func: {result3}");
}
