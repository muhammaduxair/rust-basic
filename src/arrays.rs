pub fn run() {
    // define type of data in array and total length of item
    let list: [i16; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    for i in list {
        println!("{}", i);
    }

    // The Vec<T> standard library type provides a heap-allocated resizable array type.
    let fruits: Vec<&str> = vec!["Apple", "Mango", "Cherry", "Grapes", "Papayya", "Melon"];

    for (i, x) in fruits.iter().enumerate() {
        println!("Fruit {} = {}", i, x);
    }
}
