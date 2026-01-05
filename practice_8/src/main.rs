#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

fn main() {
    let red = Color::Red;
    let green = Color::Green;
    let blue = Color::Blue;

    println!("Red: {:?}", red);
    println!("Green: {:?}", green);
    println!("Blue: {:?}", blue);
}