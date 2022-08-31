// 3.5 Control Flow

pub fn run() {
    // If expressions
    let mut number = 7;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    /*
    This fails to compile because the condition doesn't evaluate to a bool
    This is notably different from JS
    let number = 3;
    if number {
      println!("this doesn't compile!")
    }
    */

    if number != 0 {
        println!("number was something other than zero");
    }

    // Multiple conditions with else if
    number = 6;
    if number % 4 == 0 {
        println!("{number} is divisible by 4");
    } else if number % 3 == 0 {
        println!("{number} is divisible by 3");
    } else if number % 2 == 0 {
        println!("{number} is divisible by 2");
    } else {
        println!("{number} is not divisible by 4, 3, or 2");
    }

    // using if with let to assign variables
    // this same example fails to compile if the if/else arms return values of differing types
    // variables can only have a single type at compile time
    let condition = true;
    number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");

    // Loops
    // by default loop goes forever, it's up to us to make it stop when we want to
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("the result of the loop is {result}");

    // Labeling nested loops
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    // Conditional loops: while
    number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // Looping through a collection: for
    // what it looks like using a while loop
    let a = [10, 20, 30, 40 ,50];
    let mut index = 0;
    while index < a.len() {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    // a better alternative, the for loop
    for element in a {
        println!("the value is: {element}");
    }

    // for loops are much safer than the others and for that reason
    // they are the most commonly used loops.
    // even some of the previous examples, like the countdown, would
    // most often be written as a for loop using a Range
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("for loop LIFTOFF!!!");

}
