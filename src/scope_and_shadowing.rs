pub fn scope_and_shadowing() {
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