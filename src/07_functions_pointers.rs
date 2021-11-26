fn main() {
  //  01. Without the type declarations.
  let p1 = plus_one;
  let x = p1(5);

  // 02. With type declarations.
  let p1: fn(i32) -> i32 = plus_one;
  let x = p1(5); // 6
}


fn plus_one(a: i32) -> i32 {
  a + 1;
}