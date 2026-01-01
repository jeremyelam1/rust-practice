// Declare the helpers module (this tells Rust to include helpers.rs)
mod functions;
mod variables; // Declare the variables module (this tells Rust to include variables/mod.rs)
               //
fn main() {
    println!("Hello, world!");

    println!("____________________________________________");
    println!("Function Practice");
    functions::functions::execute_call_me();
    functions::functions::function_two();
    println!("############################################");
    functions::functions::func_three(4);
    let odd_response = functions::functions::func_four(15); // odo response - 12
    let even_response = functions::functions::func_four(15);
    println!("odd respnse: {odd_response}, even_response{even_response}");
    let five = functions::functions::func_five(5);
    println!("result {five}");

    println!("____________________________________________");
    // Call functions from the variables module
    println!("Variables Practice");
    variables::variables::varable_one();
    variables::variables::varable_two();
    variables::variables::variable_three();
    variables::variables::variable_four();
    variables::variables::variable_five();
    variables::variables::variable_six();
    println!("____________________________________________")
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
