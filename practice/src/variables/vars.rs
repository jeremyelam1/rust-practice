pub fn varable_one() {
    let x = 5;

    println!("x has the value {x}");
}

pub fn varable_two() {
    // TODO: Change the line below to fix the compiler error.
    let x = 10;

    if x == 10 {
        println!("x is ten!");
    } else {
        println!("x is not ten!");
    }
}

pub fn variable_three() {
    // TODO: Change the line below to fix the compiler error.
    let x: i32 = 28;

    println!("Number {x}");
}

pub fn variable_four() {
    let mut x = 3;
    println!("Number {x}");

    x = 5; // Don't change this line
    println!("Number {x}");
}

pub fn variable_five() {
    let number = "T-H-R-E-E"; // Don't change this line
    println!("Spell a number: {number}");

    // TODO: Fix the compiler error by changing the line below without renaming the variable.
    let number = 3;
    println!("Number plus two is: {}", number + 2);
}

const NUMBER: u64 = 3;
pub fn variable_six() {
    println!("Number: {NUMBER}");
}
