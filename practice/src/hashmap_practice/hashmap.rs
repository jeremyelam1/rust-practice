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
