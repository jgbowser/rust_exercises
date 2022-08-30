// 3.1 Variables and Mutability

pub fn _run() {
  /*
  this block of code doesn't compile because variables are immutable by default and so cannot be reassigned
  let x = 5;
  println!("the value of x is: {x}");
  x = 6;
  println!("the value of x is: {x}");
  */
  let mut x = 5;
  println!("the value of x is: {x}");
  x = 6;
  println!("the value of x is: {x}");

  /*
  Constants are ALWAYS immutable and must be defined with an explicit type
  they cannot be set with the result of a value computed at runtime
  */
  const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
  println!("Three hours in seconds is: {THREE_HOURS_IN_SECONDS} seconds");

  /*
  Shadowing
  Rust allows you to declare a new variable with the same name as a previous variable.
  This is known as "shadowing." The second variable overshadows the first in the same scope.
  different from mut. Can perform transformations while still maintaining immutability.
  good for type transformations (text input --> number, etc) 
  */
  let x = 5;
  let x = x + 1;
  {
    let x = x * 2;
    println!("The value of x in the inner scope is: {x}");
  }
  println!("The value of x is: {x}");
  /* this example doesn't compile because we can't reassign different types
  let spaces = "     ";
  spaces = spaces.len();
  instead we would do:
  */
  let spaces = "     ";
  let spaces = spaces.len();
  println!("The number of spaces is: {spaces}");

}