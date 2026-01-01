fn main() {
    println!("Hello, world!");

    let n1 = 1;

    println!("{}", n1);

    let no_variable = five_loop();

    println!("{}", no_variable);

    println!("{}", variable_name_loop());

    println!("{:?}", vector_slice());
}

fn five_loop() -> i32 {
    let mut counter = 0;

    loop {
        counter += 1;

        if counter == 5 {
            break counter;
        };
    }
}

fn variable_name_loop() -> i32 {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter;
        };
    };

    result
}

fn vector_slice() -> Vec<String> {
    let mut slice = vec![];
    slice.push("hello world".to_string());

    return slice;
}

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
