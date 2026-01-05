pub fn semantics_one(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(88);

    vec
}

pub fn semantics_two() {
    let vec0 = vec![22, 44, 66];

    let vec1 = semantics_one(vec0.clone());

    println!("vec0: {:?} vec1: {:?}", vec0, vec1)
}

pub fn move_semantics_three() {
    let vec0 = vec![22, 44, 66];
    let vec1 = semantics_one(vec0.clone());

    println!("og: {:?}, new: {:?}", vec0, vec1);
}

pub fn move_semantics_four() {
    let mut x = Vec::new();
    let y = &mut x;
    y.push(42);
    println!("y answer {:?}", y);

    let z = &mut x;
    z.push(13);

    println!("z answer {:?}", z)
}

// TODO: Fix the compiler errors without changing anything except adding or
// removing references (the character `&`).

// Shouldn't take ownership
fn get_char(data: &str) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(data: &str) -> String {
    data.to_uppercase()
}

pub fn move_semantics_five() {
    let data = "Rust is great!".to_string();

    get_char(&data);

    let result = string_uppercase(&data);

    println!("sem_five result: {}", result);
}
