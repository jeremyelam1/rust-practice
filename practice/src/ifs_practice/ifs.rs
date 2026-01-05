pub fn if_one_bigger(a: i32, b: i32) -> i32 {
    if a >= b {
        return a;
    }
    b
}

pub fn picky_eater(food: &str) -> &str {
    if food == "strawberry" {
        "Yummy!"
    } else {
        "1"
    }
}

pub fn animal_habitat(animal: &str) -> &str {
    // TODO: Fix the compiler error in the statement below.
    let identifier = if animal == "crab" {
        1
    } else if animal == "gopher" {
        2
    } else if animal == "snake" {
        3
    } else {
        4
    };

    // Don't change the expression below!
    if identifier == 1 {
        "Beach"
    } else if identifier == 2 {
        "Burrow"
    } else if identifier == 3 {
        "Desert"
    } else {
        "Unknown"
    }
}
