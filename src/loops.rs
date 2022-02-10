pub fn run() {
    let mut n = 0;
    let mut n2 = 0;

    loop {
        n += 1;
        println!("{}", n);
        if n == 10 {
            break;
        }
    }

    while n2 < 5 {
        n2 += 1;
        println!("{}", n);
    }

    for v in 1..100 {
        println!("{}", v)
    }
}
