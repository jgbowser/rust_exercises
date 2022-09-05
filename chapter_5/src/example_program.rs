// 5.2 Example Program Using Structs
// Calculate the area of a rectangle

pub fn _run() {
    let width = 30;
    let height = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width, height)
    );

    fn area(width: u32, height: u32) -> u32 {
        width * height
    }
    // this function is all well and good. It correctly calculates and returns the
    // area of a rectangle. We can do more to make it easier to use and read.
    // right now it isn't super clear that the 2 parameters of 'area' are related.
    // the first approach we might take is to refactor to tuples

    let rect = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels. (tuple)",
        area_with_tuple(rect)
    );
    fn area_with_tuple(dimensions: (u32, u32)) -> u32 {
        dimensions.0 * dimensions.1
    }
    // cool, now we're passing a single argument 'dimensions' in, so there
    // is already more structure in our parameters. but... it's also kind of
    // less clear. What is the width, what is the height? it is hard to tell what
    // is what with the tuple.
    // lets try using structs instead so we can give names to our values

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "Here is our struct printed out using the debug trait:\n{:#?}",
        rect2
    );

    println!(
        "The area of the rectangle is {} square pixels. (struct)",
        area_with_struct(&rect2)
    );

    fn area_with_struct(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }
    // this refactor hits the best of both worlds. We get the clear labelling
    // of the first iteration thanks to the struct fields and we get a single
    // self contained parameter to our function. This code would be much easier
    // for a different dev to quickly read and understand.

    // We've seen earlier that we can't easily just print the full contents
    // of a struct using the priontln! macro. We need to add additional functionality
    // to the struct using derived traits to do this. first instead of using '{}'
    // as the placeholder for our struct we use '{:?}' to use the 'Debug' formatter.
    // but we still need to add the Debug trait to the struct. we add #[derive(Debug)] above
    // the struct definition.
    // in the println! we can change it to '{:#?}' for pretty-print
    // we can also use the dbg! macro to print it out
    // dbg! actually takes ownership of, then returns the value passed in
    // let's give it a try

    let scale = 2;
    let rect3 = Rectangle {
        width: dbg!(30 * scale), //this value gets returned, so width = 60
        height: 50
    };
    dbg!(&rect3); // here we don't want to give dbg! ownership, so we just pass a ref

    // So this all seems good... but our area function can only really do 1 thing:
    // calculate the area of a Rectangle. It would be nice if it could live
    // right along side our struct somehow....(see 5.3: Methods)
}
