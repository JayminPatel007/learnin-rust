pub
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