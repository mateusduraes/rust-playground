fn variables() {

  let a; // Declaration; without data type
  a = 5; // Assignment
  
  let b: i8; // Declaration; with data type 
  b = 5;
  
  let t = true;        // Declaration + assignment; without data type
  let f: bool = false; // Declaration + assignment; with data type
  
  
  // Multiple declaration
  let (x, y) = (1, 2); // x = 1 and y = 2
  
  // Rust variables are immutable by default, in order to have a mutable variable, the "mut" keyword needs to be used
  
  let mut z = 5; // z is mutable
  z = 6; // z can be changed
  
  let n = { x + y }; // n = 3
  
  
  let p = {
    let x = 1;
    let y = 2;
  
    x + y
  }; // p = 3





  /*
  The const keyword is used to define constants and after the assignment their values are not allowed to change. They live for the entire lifetime of a program but has no fixed address in the memory.
  https://learning-rust.github.io/docs/a6.variable_bindings,constants_and_statics.html
  */
  const N: i32 = 5;
  
  /*
  The static keyword is used to define a “global variable” type facility. There is only one instance for each value, and it’s at a fixed location in memory.
  https://learning-rust.github.io/docs/a6.variable_bindings,constants_and_statics.html
  */
  static N: i32 = 5;
}
