// 3.3 Functions

pub fn run() {
  // naming conventions for functions and variables is snake_case.
  println!("Hello from functions.rs");

  another_function();

  // function Parameters
  // MUST declare the type of each parameter in the definition.
  parameter_function(42);

  //multiple parameters
  print_labeled_measurement(5, 'h');

  // Statements and Expressions
  // statements: instructions that perform some action and do not return a value
  // expressions: evaluate to a resulting value
  let _y = 6; // statement

  /*
  This doesn't compile because a statement doesn't return a value and so cannot
  be stored in a variable
  let x = (let y = 6);
  */
  let x = {
    let y = 3;
    y + 1 // <-- no semicolon. a semicolon turns an expression into a statement
  }; // expression
  println!("the preceeding expression returned: {x}");

  // Return values
  // return value type declared with a skinny arrow (->)
  fn five() -> i32 {
    5
  }

  let num = five();
  println!("the value returned from the function was: {num}");

  fn plus_one(num: i32) -> i32 {
    return num + 1; // can be just "num + 1" to return, or include "return" if there is a semicolon at the end
  }
  let sum = plus_one(41);
  println!("the returned value of plus_one(42) is: {sum}");
}

fn another_function() {
  println!("Hello from another function in functions.rs");
}

fn parameter_function(x: i32) {
  println!("the value of x is {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
  println!("The measurement is: {value}{unit_label}");
}