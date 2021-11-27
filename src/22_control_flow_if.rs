let age = 13;

// Only if
if age < 18 {
  println!("Child");
}

// If / else
if age < 18 {
  println!("Child")
} else {
  println!("Not a child")
}


// If with let statement 👉👉 assignes to a variable

let age: u8 = 13;
let is_below_eighteen = if age < 18 { true} else { false }; // true

// If / else if / else blocks
let team_size = 7;

if team_size < 5 {
    println!("Small");
} else if team_size < 10 {
    println!("Medium"); // The code prints this
} else {
    println!("Large");
}


// Refactoring the code above
let team_size = 7;
let team_size_in_text;

if team_size < 5 {
    team_size_in_text = "Small";
} else if team_size < 10 {
    team_size_in_text = "Medium";
} else {
    team_size_in_text = "Large";
}

println!("Current team size : {}", team_size_in_text); // Current team size : Medium

// Even more refactor, Rust is so cool 😎😎
let team_size = 7;
let team_size = if team_size < 5 { // Redeclaring the same variable so we can reassign
    "Small" // ⭐️ no ;
} else if team_size < 10 {
    "Medium"
} else {
    "Large"
};

println!("Current team size : {}", team_size); // Current team size : Medium