let a = [1, 2, 3];
let a: [i32, 3] = [1, 2, 3]; // [Type, NO of elements]

let b: [i32, 0] = []; // An empty array

let muc c: [132; 3] = [1, 2, 3];
c[0] = 2;
c[1] = 4;
c[2] = 6;

// The array above is composed of 3 elements of type i32. Using `mut` you can change the elements the array size is always immutable.

let d = [0; 5];   // [0, 0, 0, 0, 0]
let e = ["x"; 5]; // ["x", "x", "x", "x", "x"]