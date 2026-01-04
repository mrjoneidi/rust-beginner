fn main() {
    // look practice_1, you can make this code to get numbers from user
    let num1 = 10;
    let num2 = 5;

    // Addition
    let addition_result = num1 + num2;
    println!("Addition result: {}", addition_result);

    // Subtraction
    let subtraction_result = num1 - num2;
    println!("Subtraction result: {}", subtraction_result);

    // Multiplication
    let multiplication_result = num1 * num2;
    println!("Multiplication result: {}", multiplication_result);

    // Division: with check 0 
    if num2 != 0 {
        let division_result = num1 / num2;
        println!("Division result: {}", division_result);
    } else {
        println!("Cannot divide by zero!");
    }
}
