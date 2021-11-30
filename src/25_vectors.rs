// How to create an empty vector
let mut a = Vec::new(); // 1. With new() keyword
let mut b = vec![]; // 2. Using the vec! macro

// Create a vector with data types
let mut a2:  Vec<i32> = Vec::new();
let mut b2:  Vec<i32> = vec![];
let mut b3 = vec![1i32, 2, 3]; // Sufixing the first value with the data type

// Access and change data
let mut c = vec![5, 4 , 3, 2 , 1];
c[0] = 1;
c[1] = 2;
// c[6] = 2; Cannot assign values this way, index out of bounds

// Push and pop
let mut d: Vec<i32> = Vec::new();
d.push(1);
d.push(2);
d.pop(); // [1] It will remove an element from the end

// 🔎 Capacity and reallocation
let mut e: Vec<i32> = Vec::with_capacity(10);
printn!("Length: {}, Capacity: {}", e.len(), e.capacity()); // Length: 0, Capacity: 10;
/*
    Capacity vs Length
        Capacity is the amount of space allocated for any future elements.
        Length is the number of elements currently allocated in the Vector.
*/

// These are all done without reallocating...
for i in 0..10 {
    e.push(i);
}

// ...but this may make the vector reallocate
e.push(11);