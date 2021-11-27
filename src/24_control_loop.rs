loop {
    println!("Loop forever!"); // 😂 -> While true?
}


// Usage of break and continue
let mut a = 0;

loop {
    if a == 0 {
        println!("Skip Value : {}", a);
        a += 1;
        continue;
    } else if a == 2 {
        println!("Break At : {}", a);
        break;
    }

    println!("Current Value : {}", a);
    a += 1;
}

// Outer break
let mut b1 = 1;

'outer_loop: loop { //set label outer_loop
  let mut b2 = 1;

  'inner_loop: loop {
    println!("Current Value : [{}][{}]", b1, b2);

    if b1 == 2 && b2 == 2 {
        break 'outer_loop; // kill outer_loop
    } else if b2 == 5 {
        break;
    }

    b2 += 1;
  }

  b1 += 1;
}



// Using while
let mut a = 1;

while a <= 10 {
    println!("Current value : {}", a);
    a += 1; //no ++ or -- on Rust
}


// Usage of break and continue
let mut b = 0;

while b < 5 {
    if b == 0 {
        println!("Skip value : {}", b);
        b += 1;
        continue;
    } else if b == 2 {
        println!("Break At : {}", b);
        break;
    }

    println!("Current value : {}", b);
    b += 1;
}

// Outer break
let mut c1 = 1;

'outer_while: while c1 < 6 { //set label outer_while
    let mut c2 = 1;

    'inner_while: while c2 < 6 {
        println!("Current Value : [{}][{}]", c1, c2);
        if c1 == 2 && c2 == 2 { break 'outer_while; } //kill outer_while
        c2 += 1;
    }

    c1 += 1;
}


// 0 to 10 (10 exclusive); In other languages, `for(i = 0; i < 10; i++)`
for i in 0..10 {
    println!("Current value : {}", i);
}

// 1 to 10 (10 inclusive); In other languages, `for(i = 1; i <= 10; i++)`
for i in 1..=10 {
    println!("Current value : {}", i);
}

// Usage of break and continue
for b in 0..6 {
    if b == 0 {
      println!("Skip Value : {}", b);
      continue;
    } else if b == 2 {
      println!("Break At : {}", b);
      break;
    }
  
    println!("Current value : {}", b);
}

// Outer break
'outer_for: for c1 in 1..6 { //set label outer_for

    'inner_for: for c2 in 1..6 {
      println!("Current Value : [{}][{}]", c1, c2);
      if c1 == 2 && c2 == 2 { break 'outer_for; } //kill outer_for
    } 

}




// Working with arrays/vectors
let group : [&str; 4] = ["Mark", "Larry", "Bill", "Steve"];

for n in 0..group.len() { // group.len() = 4 -> 0..4 👎 check group.len()on each iteration
  println!("Current Person : {}", group[n]);
}

for person in group.iter() { // 👍 group.iter() turn the array into a simple iterator
  println!("Current Person : {}", person);
}