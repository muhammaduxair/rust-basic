// you can define any name of the function
pub fn run() {
    let name = "john";
    let mut age = 20;

    println!("{} age is {}", name, age);
    age = 25;
    println!("{} age is {}", name, age);

    // Define contant variable
    const ID: i32 = 0001;
    println!("ID: {}", ID);

    // assign multiple variables
    let (my_name, my_age) = ("Doe", 35);
    print!("{} is {}", my_name, my_age);
}
