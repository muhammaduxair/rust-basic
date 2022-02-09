pub fn run() {
    println!("hello from prints.rs file");

    println!("Number: {}", 1);

    println!("{} is from {}", "john", "Mass");

    println!(
        "{0} is {1} and {0} like {2}",
        "john", "developer", "development"
    );

    println!(
        "{name} is like {activity}",
        name = "john",
        activity = "cricket"
    );

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // basic math
    println!("10 + 10 = {}", 10 + 10)
}
