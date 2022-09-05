// 5.3 Method Syntax

pub fn run() {
    // let's start by bringing in the final product of our previous section
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    // here we add our area method to our Rectangle struct
    // the first parameter of a method is always 'self' which
    // is a lot like 'this' in javascript in that it is a reference
    // to the struct that the method is a part of
    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }
    // We’ve chosen &self here for the same reason we used &Rectangle in the
    // function version: we don’t want to take ownership, and we just want to
    // read the data in the struct, not write to it. If we wanted to change the
    // instance that we’ve called the method on as part of what the method does,
    // we’d use &mut self as the first parameter. Having a method that takes
    // ownership of the instance by using just self as the first parameter is rare;
    // this technique is usually used when the method transforms self into something else
    // and you want to prevent the caller from using the original instance after the transformation.

    let rect = Rectangle {
        width: 60,
        height: 100,
    };

    println!(
        "The area of the rectangle is {} square pixels. (method)",
        rect.area()
    );

    // methods can also have the same name as fields in the struct
    impl Rectangle {
        fn width(&self) -> bool {
            self.width > 0
        }
    }

    if rect.width() {
        println!("The rectangle has a non-zero width; it is {}", rect.width);
    }
    // this type of naming is often used for 'getter' methods where perhaps
    // width is a private field, but we have a getter method to allow access to it.

    // Methods with More Parameters
    // let's add a new method to rectangle that will return a bool based on
    // whether a second given rectangle fits within 'self'

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect3 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect4 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect2 hold rect3? {}", rect2.can_hold(&rect3));
    println!("Can rect2 hold rect4? {}", rect2.can_hold(&rect4));

    impl Rectangle {
        fn can_hold(&self, rect: &Rectangle) -> bool {
            self.width > rect.width && self.height > rect.height
        }
    }

    // Associated Functions
    // all functions defined in an `impl` block are associated functions
    // not all associated functions have `self` as their first parameter.
    // when this is the case they are often used as constructors to return
    // a new instance of the struct, these are often called `new`.
    // lets try this with a method called `square` that allows us to make
    // a square instance of Rectangle easily
    impl Rectangle {
        fn square(size: u32) -> Self {
            Self {
                width: size,
                height: size,
            }
        }
    }

    let square = Rectangle::square(25);
    println!(
        "here is the square instance of Rectangle created using .square()\n{:#?}",
        square
    );
}
