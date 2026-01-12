mod enums_practice;
mod functions;
mod ifs_practice;
mod modules_practice;
mod move_semantics;
mod primitive;
mod strings_practice;
mod structures;
mod variables; // Declare the variables module (this tells Rust to include variables/mod.rs)
mod vector;
//
fn main() {
    println!("Hello, world!");
    println!("____________________________________________");
    println!("modules practice");
    modules_practice::modules_one();
    modules_practice::modules_two();
    modules_practice::modules_three();

    let run = false;
    if run {
        println!("____________________________________________");
        println!("Function Practice");
        functions::execute_call_me();
        functions::function_two();
        println!("############################################");
        functions::func_three(4);
        let odd_response = functions::func_four(15); // odo response - 12
        let even_response = functions::func_four(15);
        println!("odd respnse: {odd_response}, even_response{even_response}");
        let five = functions::func_five(5);
        println!("result {five}");

        println!("____________________________________________");
        println!("string practice");

        let str_one = strings_practice::string_one();
        println!("string one result: {}", str_one);

        strings_practice::string_two();
        strings_practice::string_three();

        println!("____________________________________________");
        println!("enum practice");

        enums_practice::enum_one();

        println!("********************************************");
        enums_practice::enum_two();

        println!("********************************************");
        enums_practice::enum_three();
        println!("____________________________________________");
        println!("structure practice");

        structures::structs_one();
        structures::strtucts_two();

        let structs_three = structures::Package::new(String::from("usa"), String::from("usa"), 4);

        let international = structs_three.is_international();
        println!("{}", international);

        let price = structs_three.get_fees(40);
        println!("price is: {}", price);

        println!("{}", structs_three.sender_country);

        println!("____________________________________________");
        println!("structure practice");

        structures::structs_one();
        structures::strtucts_two();

        let structs_three = structures::Package::new(String::from("usa"), String::from("usa"), 4);

        let international = structs_three.is_international();
        println!("{}", international);

        let price = structs_three.get_fees(40);
        println!("price is: {}", price);

        println!("{}", structs_three.sender_country);
        println!("____________________________________________");
        println!("semantics practice");

        let sem_one = move_semantics::semantics_one(vec![1, 2, 3, 4, 5]);
        move_semantics::semantics_two();
        move_semantics::move_semantics_three();
        move_semantics::move_semantics_four();
        move_semantics::move_semantics_five();

        println!("{ :? }", sem_one);
        println!("____________________________________________");
        println!("vec practice");
        let result = vector::vec_one();
        println!("{:?}", result);
        let two = vec![2, 3, 4, 5];
        let two_result = vector::vec_two(&two);
        println!("{:?}", two_result);
        println!("____________________________________________");
        println!("Primitive Practice");
        primitive::primitive_bool();
        primitive::primitive_two();
        primitive::prim_three();
        primitive::prim_five();
        primitive::prim_six();
        println!("____________________________________________");
        println!("if Practice");
        let b_bigger = ifs_practice::if_one_bigger(10, 11); // 11 should be returned
        let a_bigger = ifs_practice::if_one_bigger(12, 11); // 12 should be returned
        println!("b is bigger {b_bigger}, a is bigger {a_bigger}");
        let picky = ifs_practice::picky_eater("strawberry");
        println!("{picky}");
        let habit = ifs_practice::animal_habitat("snake");
        println!("{habit}");

        println!("____________________________________________");
        println!("Function Practice");
        functions::execute_call_me();
        functions::function_two();
        println!("############################################");
        functions::func_three(4);
        let odd_response = functions::func_four(15); // odo response - 12
        let even_response = functions::func_four(15);
        println!("odd respnse: {odd_response}, even_response{even_response}");
        let five = functions::func_five(5);
        println!("result {five}");

        println!("____________________________________________");
        // Call functions from the variables module
        println!("Variables Practice");
        variables::varable_one();
        variables::varable_two();
        variables::variable_three();
        variables::variable_four();
        variables::variable_five();
        variables::variable_six();
        println!("____________________________________________")
    }
}
