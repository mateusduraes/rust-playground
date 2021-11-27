let a: = "Hello, world."; // a: &'static str
let b: &str = "こんにちは, 世界!";

/*
⭐️ It’s an immutable/ statically allocated slice holding an unknown sized sequence of UTF-8 code points stored in somewhere in memory. &str is used to borrow and assign the whole array to the given variable binding.
https://learning-rust.github.io/docs/a8.primitive_data_types.html#str
*/