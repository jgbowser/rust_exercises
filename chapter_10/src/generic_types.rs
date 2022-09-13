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

    // In Struct Definitions
    // we can also define structs to use generics as well
    struct Point<T> {
        x: T,
        y: T,
    }

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.1, y: 4.2 };
    println!("x in the integer version is: {}", integer.x);
    println!("y in the float version is: {}", float.y);
    // we only defined Point with a single type parameter <T>, so all fields
    // must be of the same type, we can specify multiple types though
    #[derive(Debug)]
    struct OtherPoint<T, U> {
        _x: T,
        _y: U,
    }
    // now these are all valid
    let both_integer = OtherPoint { _x: 5, _y: 10 };
    let both_float = OtherPoint { _x: 1.2, _y: 5.6 };
    let integer_and_float = OtherPoint { _x: 5, _y: 8.9 };
    println!(
        "here are all the different forms now: {:?}, {:?}, {:?}",
        both_integer, both_float, integer_and_float
    );

    // In Enum Definitions
    // just like structs we can define enums with generic types

    // Options are an example
    enum _Option<T> {
        Some(T),
        None,
    }

    // In Method Definitions
    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());
    // we can also specify constraints on generic types when implementing methods
    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    let fp = Point { x: 4.0, y: 10.5 };

    println!("distance from origin fp = {}", fp.distance_from_origin());
    // we can implement methods with generics that don't match the struct definition
    struct AnotherPoint<X1, Y1> {
        x: X1,
        y: Y1,
    }

    impl<X1, Y1> AnotherPoint<X1, Y1> {
        fn mixup<X2, Y2>(self, other: AnotherPoint<X2, Y2>) -> AnotherPoint<X1, Y2> {
            AnotherPoint {
                x: self.x,
                y: other.y,
            }
        }
    }

    let p1 = AnotherPoint { x: 5, y: 10.4 };
    let p2 = AnotherPoint { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
