use std::mem;

pub fn data_types() {
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