use time::OffsetDateTime;

fn main() {
    println!("Enter your birth year:");
    let mut b_year = String::new();
    std::io::stdin().read_line(&mut b_year).expect("Failed to read input.");
    let b_year: i32 = b_year.trim().parse().expect("Please enter a valid number.");

    let now = OffsetDateTime::now_utc();
    let current_year = now.year();
    println!("The current year is: {}", current_year);

    println!("your age is: {}", current_year-b_year);
}