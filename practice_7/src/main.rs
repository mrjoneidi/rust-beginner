struct Person {
    first_name: String,
    last_name: String,
    age: u32,
    organization: String,
}

fn main() {
    let person1= Person {
        first_name: String::from("Mory"),
        last_name: String::from("JJ"),
        age: 23,
        organization: String::from("KNTU"),
    };
    let _person2= Person {
        first_name: String::from("Amir"),
        last_name: String::from("Hoseini"),
        age: 25,
        organization: String::from("Nvidia"),
    };
    
    println!("Person1's name is: {} {}", person1.first_name, person1.last_name);
    println!("His age is {}", person1.age);
    println!("He works at {}", person1.organization);
}
