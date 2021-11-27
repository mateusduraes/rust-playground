let tshirt_width = 20;
let tshirt_size = match tshirt_width {
    16 => "S", // check 16
    17 | 18 => "M", // check 17 and 18
    19 ..= 21 => "L", // check from 19 to 21 (19,20,21)
    22 => "XL",
    _ => "Not Available", // default value
};

println!("{}", tshirt_size); // L


// Using a boolean, there are only two possible values (true/false), so no need for a default value
let is_allowed = false;
let list_type = match is_allowed {
    true => "Full",
    false => "Restricted"
    // no default/ _ condition can be skipped
    // Because data type of is_allowed is boolean and all possibilities checked on conditions
};

println!("{}", list_type); // Restricted


// Using match with tuples
let marks_paper_a: u8 = 25;
let marks_paper_b: u8 = 30;

// The tuple here is (25, 30)
let output = match (marks_paper_a, marks_paper_b) {
    (50, 50) => "Full marks for both papers",
    (50, _) => "Full marks for paper A",
    (_, 50) => "Full marks for paper B",
    (x, y) if x > 25 && y > 25 => "Good", // This is dynamic, checks if 'x' and 'y' are greather than 25
    (_, _) => "Work hard" // Default value for both items in the tuple
};

println!("{}", output); // Work hard