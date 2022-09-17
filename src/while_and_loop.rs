pub fn while_and_loop() {
    // Code with while loop
    let mut x = 1;
    while x < 1000 {
        x *= 2;
        if (x == 64) { continue; }
        println!("x = {}", x);
    }

    // Code with loop
    let mut y = 1;
    loop {
        y *= 2;
        println!("y = {}", y);
        if y > 1000 { break; }
    }
}