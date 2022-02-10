enum Directions {
    Up,
    Down,
    Left,
    Right,
}

pub fn run() {
    let direction: Directions = Directions::Up;

    match direction {
        Directions::Up => println!("Direction is Up"),
        Directions::Down => println!("Direction is Down"),
        Directions::Left => println!("Direction is Left"),
        Directions::Right => println!("Direction is Right"),
    }
}
