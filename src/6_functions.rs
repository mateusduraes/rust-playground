// Hello world
fn main () {
  println!("Hello, world!");
}

// Passing arguments
fn print_sum(a: i8, b: i8) {
  println!("sum is: {}", a + b);
}


// Returning values 
/*
  Without the return keyword. Only the last expression returns.
*/
fn plus_one(a: i32) -> i32 {
  a + 1
  // There is no ending ; in the above line.
  // It means this is an expression which equals to `return a + 1;`.
}

// 02. With teh return keyword
fn plus_two(a: i32) -> i32 {
  return a + 2;
  // Should use return keyword only on conditional/ early returns.
  // Using return keyword in the last expression is a bad practice.
}
