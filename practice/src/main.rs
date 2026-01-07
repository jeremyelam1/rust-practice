mod functions;
mod ifs_practice;
mod move_semantics;
mod primitive;
mod structures;
mod variables; // Declare the variables module (this tells Rust to include variables/mod.rs)
mod vector;
//
fn main() {
    println!("Hello, world!");

    println!("____________________________________________");
    println!("structure practice");

    structures::structs::structs_one();
    structures::structs::strtucts_two();

    let structs_three =
        structures::structs::Package::new(String::from("usa"), String::from("usa"), 4);

    let international = structs_three.is_international();
    println!("{}", international);

    let price = structs_three.get_fees(40);
    println!("price is: {}", price);

    println!("{}", structs_three.sender_country);

    let run = false;
    if run {
        println!("____________________________________________");
        println!("semantics practice");

        let sem_one = move_semantics::semantics::semantics_one(vec![1, 2, 3, 4, 5]);
        move_semantics::semantics::semantics_two();
        move_semantics::semantics::move_semantics_three();
        move_semantics::semantics::move_semantics_four();
        move_semantics::semantics::move_semantics_five();

        println!("{ :? }", sem_one);
        println!("____________________________________________");
        println!("vec practice");
        let result = vector::vec::vec_one();
        println!("{:?}", result);
        let two = vec![2, 3, 4, 5];
        let two_result = vector::vec::vec_two(&two);
        println!("{:?}", two_result);
        println!("____________________________________________");
        println!("Primitive Practice");
        primitive::prim::primitive_bool();
        primitive::prim::primitive_two();
        primitive::prim::prim_three();
        primitive::prim::prim_five();
        primitive::prim::prim_six();
        println!("____________________________________________");
        println!("if Practice");
        let b_bigger = ifs_practice::ifs::if_one_bigger(10, 11); // 11 should be returned
        let a_bigger = ifs_practice::ifs::if_one_bigger(12, 11); // 12 should be returned
        println!("b is bigger {b_bigger}, a is bigger {a_bigger}");
        let picky = ifs_practice::ifs::picky_eater("strawberry");
        println!("{picky}");
        let habit = ifs_practice::ifs::animal_habitat("snake");
        println!("{habit}");

        println!("____________________________________________");
        println!("Function Practice");
        functions::func::execute_call_me();
        functions::func::function_two();
        println!("############################################");
        functions::func::func_three(4);
        let odd_response = functions::func::func_four(15); // odo response - 12
        let even_response = functions::func::func_four(15);
        println!("odd respnse: {odd_response}, even_response{even_response}");
        let five = functions::func::func_five(5);
        println!("result {five}");

        println!("____________________________________________");
        // Call functions from the variables module
        println!("Variables Practice");
        variables::var::varable_one();
        variables::var::varable_two();
        variables::var::variable_three();
        variables::var::variable_four();
        variables::var::variable_five();
        variables::var::variable_six();
        println!("____________________________________________")
    }
}

// fn five_loop() -> i32 {
//     let mut counter = 0;
//
//     loop {
//         counter += 1;
//
//         if counter == 5 {
//             break counter;
//         };
//     }
// }
//
// fn variable_name_loop() -> i32 {
//     let mut counter = 0;
//
//     let result = loop {
//         counter += 1;
//
//         if counter == 10 {
//             break counter;
//         };
//     };
//
//     result
// }
//
// fn vector_slice() -> Vec<String> {
//     let mut slice = vec![];
//     slice.push("hello world".to_string());
//
//     return slice;
// }
//
// fn vector_int32() -> Vec<i32> {
//     let mut slice = vec![];
//     for i in 0..6 {
//         slice.push(i)
//     }
//
//     slice
// }
//
// fn collect_vector_int32() -> Vec<i32> {
//     (-10..8).collect()
// }

// Example struct
// struct Person {
//     name: String,
//     age: u32,
//     email: String,
// }
//
// // Implementation block - methods attached to the struct
// impl Person {
//     // Associated function (constructor) - called with Person::new()
//     fn new(name: String, age: u32, email: String) -> Person {
//         Person { name, age, email }
//     }
//
//     // Method that borrows self immutably
//     fn introduce(&self) {
//         println!("Hi, I'm {} and I'm {} years old.", self.name, self.age);
//     }
//
//     // Method that borrows self immutably and returns a value
//     fn get_email(&self) -> &str {
//         &self.email
//     }
//
//     // Method that mutably borrows self
//     fn have_birthday(&mut self) {
//         self.age += 1;
//         println!("{} is now {} years old!", self.name, self.age);
//     }
//
//     // Method that checks a condition
//     fn is_adult(&self) -> bool {
//         self.age >= 18
//     }
//
//     // Method that consumes self (takes ownership)
//     fn into_name(self) -> String {
//         self.name
//     }
// }
