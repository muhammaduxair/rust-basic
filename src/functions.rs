pub fn run() {
    print_numbers_in_loop(20);
}

fn print_numbers_in_loop(n: i32) {
    for v in 0..n {
        let mut x = "";
        if is_odd(v) {
            x = "Odd";
        } else {
            x = "Even"
        }
        println!("Number :{} is {}", v, x);
    }
}

fn is_odd(num: i32) -> bool {
    return num % 2 == 0;
}
