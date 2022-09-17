pub fn match_statement() {
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