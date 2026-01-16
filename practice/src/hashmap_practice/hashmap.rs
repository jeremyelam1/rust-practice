use std::collections::HashMap;

pub fn hashmap_one() -> HashMap<String, u32> {
    // TODO: Declare the hash map.
    // let mut basket =

    let mut basket = HashMap::new();

    // Two bananas are already given for you :)
    basket.insert(String::from("banana"), 2);

    // TODO: Put more fruits in your basket.
    basket.insert(String::from("apple"), 4);
    basket.insert(String::from("grapes"), 4);

    basket
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Fruit {
    Apple,
    Banana,
    Mango,
    Lychee,
    Pineapple,
}

pub fn hash_two(basket: &mut HashMap<Fruit, u32>) {
    let fruit_kinds = [
        Fruit::Apple,
        Fruit::Banana,
        Fruit::Mango,
        Fruit::Lychee,
        Fruit::Pineapple,
    ];

    for fruit in fruit_kinds {
        // TODO: Insert new fruits if they are not already present in the
        // basket. Note that you are not allowed to put any type of fruit that's
        // already present!
        //
        if let Some(value) = basket.get_mut(&fruit) {
            *value += 1;
        } else {
            basket.insert(fruit, 1);
        }
    }
}
