use std::mem;

const MEANING_OF_LIFE:u8 = 52; // no fixed address
static Z:i32 = 123;
static mut MUT_Z:i32 = 456;

fn main() {
    // data_types();
    // bitwise_operators();
    // logical_operator();
    // scope_and_shadowing();
    // constants();
    // stack_and_heap_memory();
    // if_statement();
    // while_and_loop();
    // for_loop();
    match_statement();
}

fn match_statement() {
    let country_code = 999;
    let country = match country_code {
        44 => "UK",
        91 => "IN",
        7 => "Russia",
        1..=1000 => "Unknown",
        _ => "Invalid"
    };
    println!("{} with code {}", country, country_code);
}

fn for_loop() {
    for x in 1..11 {
        if x == 3 {continue;}
        if x == 8 {break;}
        println!("x = {}", x);
    }

    for (pos, y) in (30..41).enumerate() {
        println!("{}th value is {}", pos + 1, y);
    }
}

fn while_and_loop() {
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

fn if_statement() {
    let temp = 35;
    if temp > 30 {
        println!("Hot");
    } else if temp < 10 {
        println!("Cold");
    } else {
        println!("Temperature is OK");
    }

    let day = if temp > 20 { "sunny" } else { "Cloudy"}; // if condition as expression
    println!("Today is {}", day);

    println!("it is {}",
        if temp > 20 {"Hot"} else if temp < 10 {"Cold"} else {"ok"}); // Expression in macro

    println!("it is {}",
        if temp > 20 {
            if temp > 30 {"very hot"} else {"hot"}
        } else {"Cold"}) // We can also use nested if in expression
}

struct Point {
    x: f64,
    y: f64
}

fn origin() -> Point {
    Point{x: 0.0, y: 0.0}
}

fn stack_and_heap_memory() {
    let x = 5; // Allocated in stack
    let y = Box::new(6); // value is stored in heap and address variable is created in stack

    // Points
    let p1 = origin(); // point in Stack
    let p2 = Box::new(origin()); // Point in Heap

    println!("p1 takes {} bytes", mem::size_of_val(&p1)); // will print 16 bytes
    println!("p2 takes {} bytes", mem::size_of_val(&p2)); // will print 8 bytes

    let p3 = *p2; // Value from p2 is copied and new point in Stack is created, REFERENCE IS NOT SHARED HERE
}

fn constants() {
    println!("Meaning of life = {}", MEANING_OF_LIFE); // MEANING_OF_LIFE will be replaced by 42 at compile time
    println!("Z = {}", Z); // Z will have memory address
    // println!("MUT_Z = {}", MUT_Z); // will give compile time error for Memory safety
    // MUT_Z = 777; // This will also give error
    unsafe {
        println!("MUT_Z = {}", MUT_Z); // will print 456;
        MUT_Z = 777;
        println!("MUT_Z = {}", MUT_Z); // will print 777;
    }

}

fn scope_and_shadowing() {
    let a = 123;
    let a = 777; // Re-declaring the variable doesn't give compile time error

    // separate scope
    {
        let b = 555;
        println!("inside b = {}", b);

        println!("first: inside a = {}", a); // a = 777

        let a = 987; // shadowing a variable
        println!("second: inside a = {}", a); // a = 987
    }

    println!("a = {}", a); // a = 777;
    // println!("b = {}", b); // can not access b outside of scope
}

fn logical_operator() {
    let pi_less_than_4 = std::f64::consts::PI < 4.0; // < & >
    println!("pi less than 4 ? = {}", pi_less_than_4);

}

fn bitwise_operators() {
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

fn arithmetic_operators() {
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

fn data_types() {
    let a:u8 = 123;
    println!("a = {}", a);

    let mut b:u8 = 100;
    println!("b = {}", b);
    b = 45;
    println!("b = {}", 45);

    let c = 123456789;
    println!("c = {}, size = {} byte(s)", c, mem::size_of_val(&c));

    let z:isize = 123; // for 32-bit system isize=32 bit and for 64-bit system isize=64 bits
    println!("z = {}, size = {} byte(s)", z, mem::size_of_val(&z));

    let d = 'x';
    println!("d = {}, size = {} byte(s)", d, mem::size_of_val(&d));

    let e = 2.5; // double precision
    println!("e = {}, size = {} byte(s)", e, mem::size_of_val(&e));

    let f = false; // 1 byte size
    println!("f = {}, size = {} byte(s)", f, mem::size_of_val(&f));
}
