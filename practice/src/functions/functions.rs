pub fn execute_call_me() {
    call_me();
}

fn call_me() {
    println!("i have been called!")
}

pub fn function_two() {
    let num = 3;
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

pub fn func_three(num: u8) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

pub fn func_four(price: i64) -> i64 {
    if is_even(price) {
        price - 10
    } else {
        price - 3
    }
}

pub fn func_five(num: i32) -> i32 {
    num * num
}

fn is_even(num: i64) -> bool {
    num % 2 == 0
}
