fn main() {
    println!("Enter a number:");
    // these 3 lines are for getting inout "number" from user
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input.");
    let number: i32 = input.trim().parse().expect("Please enter a valid number.");

    if number%2==0{
        println!("The number {} is Even.", number);
    } else {
        println!{"The number {} is Odd.", number};
    }
}
