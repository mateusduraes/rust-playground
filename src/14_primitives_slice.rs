/*
Dynamically-sized reference to another data structure
Imagine you want to get/ pass a part of an array or any other data structure. Instead of copying it to another array (or same data structure), Rust allows for creating a view/ reference to access only that part of the data. This view/ reference can be mutable or immutable.
https://learning-rust.github.io/docs/a8.primitive_data_types.html#Slice
*/


let a: [i32; 4] = [1, 2, 3, 4]; // Parent Array

let b: &[i32] = &a; // Slicing whole array
let c = &a[0..4]; // From 0th position to 4th(excluding)
let d = &a[..]; // Slicing whole array

let e = &a[1..3]; // [2, 3]
let f = &a[1..]; // [2, 3, 4]
let g = &a[..3]; // [1, 2, 3]