// Named function
fn main() {
  let x = 2;
  println!("{}", get_square_value(x));
}

fn get_square_value(i: i32) -> i32 {
  i * i
}

// With optiona ltype declarations of input and return types
fn main() {
  let x = 2;
  let square = |i: i32| -> i32 {  // Input parameters are passed inside | | and expression body is wrapped within { }
    i * i
  }
  println!("{}", square(x))
}

// Without type declarations of input and return types
fn main() {
  let x = 2;
  let square = |i| i * i; // { } are op tional for single-liend closures
  printl!("{}", square(x));
}


// With optional type declarations; Creating and calling together
fn main() {
  let x = 2;
  let x_square = |i: i32| -> i32 {  i * i }(x); // { } are mandatory while creating and calling same time
  println!("{}", x_square);
}

// Without optional type declarations; Creating and calling together
fn main() {
  let x = 2;
  let x_square = |i| -> i32 { i * i}(x); // ⭐️ The return type is mandatory.
  println!("{}", x_square);
}