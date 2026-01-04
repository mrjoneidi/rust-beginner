fn main() {
    println!("Enter a number for the start of range:");
    let mut input1 = String::new();
    std::io::stdin().read_line(&mut input1).expect("Failed to read input.");
    let number1: i32 = input1.trim().parse().expect("Please enter a valid number.");

    println!("Enter a number for the end of range:");
    let mut input2 = String::new();
    std::io::stdin().read_line(&mut input2).expect("Failed to read input.");
    let number2: i32 = input2.trim().parse().expect("Please enter a valid number.");

    for i in number1..=number2 {
        if i%2==0{
            println!("{}", i);
        } else{
            continue;
        }
    }
}
