const MEANING_OF_LIFE:u8 = 52; // no fixed address
static Z:i32 = 123;
static mut MUT_Z:i32 = 456;

pub fn constants() {
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
