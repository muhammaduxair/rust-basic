pub fn run() {
    let mut a = 10;

    // creatign a refrence
    let b = &mut a;

    *b += 1;

    println!("b is {}", a);
}
