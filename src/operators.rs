pub fn arithmetic_operators() {
    let mut a = 2 + 3 * 4;
    println!("a = {}", a);
    // a++; // this doesn't work
    a = a + 1;
    a += 2;
    println!("a = {}", a);
    let a_cubed = i32::pow(a, 3);
    println!("a_cubed = {}", a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);
    println!("{} cubed = {}", b, b_cubed);
    println!("{} to power PI = {}", b, b_to_pi);
}

pub fn bitwise_operators() {
    let a = 1 | 2; // | OR
    let b = 1 & 2; // & AND
    let c = 1 ^ 2; // ^ XOR
    let d = ! 1;   // ! NOT
    println!("a = {}", a);
    println!("b = {}", b);
    println!("c = {}", c);
    println!("d = {}", d);

    // shift operator
    let two_to_10 = 1 << 10; // << & >>
    println!("2^10 = {}", two_to_10);
}

pub fn logical_operator() {
    let pi_less_than_4 = std::f64::consts::PI < 4.0; // < & >
    println!("pi less than 4 ? = {}", pi_less_than_4);

}
