struct RGBColor {
    red: i16,
    green: i16,
    blue: i16,
}

pub fn run() {
    let my_color = RGBColor {
        red: 2356,
        green: 244,
        blue: 100,
    };

    println!(
        "My Color RGB Color is {},{},{}",
        my_color.red, my_color.green, my_color.blue
    );
}
