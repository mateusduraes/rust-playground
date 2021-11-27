// Fixed size ordered list of elements of different(or same) data types

let a = (1, 1.5, true, 'a'); // Without defining type
let a: (i32, f64, bool, char) = (1, 1.5, true, 'a'); // Defining type

let mut b = (1, 1.5);
b.0 = 2;
b.1 = 3.0;

println!("{:?}", b); // (2, 3.0)
println!("{:#?}", b);
// (
//   2,
//   3.0,
// )


// Assigning new variables from existing tuple
let (c, d) = b; // c = 2, d = 3.0


// Assigning new variables from existing tuple and skip uneeded elements
let (e, _, _, f) = a; // e = 1, f = 'a'
