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
