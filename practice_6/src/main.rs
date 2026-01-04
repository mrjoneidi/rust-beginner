// factorial function
fn factorial(n: u64) -> u64 {
    let mut result = 1;

    for i in 1..=n {
        result*=i;
    }

    result
}


fn main() {
    println!("Enter a number for calculating factorial:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input.");
    let number: u64 = input.trim().parse().expect("Please enter a valid number.");

    let factorial_result = factorial(number);

    println!("Factorial of {} is: {}", number, factorial_result);
}