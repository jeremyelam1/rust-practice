pub fn string_one() -> String {
    "blue".to_string()
}

pub fn string_two() {
    let word = String::from("green"); // Don't change this line.

    if is_a_color_word(&word) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}

fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}

fn trim_me(input: &str) -> &str {
    // TODO: Remove whitespace from both ends of a string.
    input.trim()
}

fn compose_me(input: &str) -> String {
    // format!("{}, world!", input)
    input.to_string() + ", world!"
    // TODO: Add " world!" to the string! There are multiple ways to do this.
}

fn replace_me(input: &str) -> String {
    input.replace(input, "ballons")
    // TODO: Replace "cars" in the string with "balloons".
}

pub fn string_three() {
    println!("compse: {}", compose_me("hello"));
    println!(
        "trim: {}",
        trim_me("                         this is to be trim without spaces                 ")
    );
    println!(
        "replace_me: old word elaine new word {}",
        replace_me("elaine")
    )
}
