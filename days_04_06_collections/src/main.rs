use counter::Counter;
use std::collections::HashMap;

fn main() {
    let emp = ("Bob", 35);
    println!("Employee {}'s salary is {}", emp.0, emp.1);

    //Tuple Destructing

    let emp = (1, "Bob", 35);

    let (a, b, c) = emp;

    //Order is important,  B comes infront of A in the print statement
    println!("Employee {}'s salary is {} and id {}  ", b, c, a);

    let name = "Bob";
    let salary = 37.0;
    let id = 10;
    let emp = Employee { name, salary, id };

    println!(
        "Employee {}'s salary is {} and id {}  ",
        emp.name, emp.salary, emp.id
    );

    //2 . Rust hashmap usage

    let mut users = HashMap::new();

    users.insert("Bob".to_string(), "Coder".to_string());

    match users.get("Bob") {
        Some(profession) => println!("Bob is a  {}", profession),
        None => println!("Bob has no known profession"),
    }

    match users.get("Ken") {
        Some(profession) => println!("Ken is a  {}", profession),
        None => println!("Ken has no known profession"),
    }

    users.insert("Robin".to_string(), "Tester".to_string());

    // Iterate over everything.
    for (name, profession) in &users {
        println!("{}: \"{}\"", name, profession);
    }

    let words = "Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been 
the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and 
scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into 
electronic typesetting, remaining essentially unchanged. It was popularised in the 1960s with the release of
Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus
PageMaker including versions of Lorem Ipsum";

    let common_words = words
        .split_whitespace()
        .collect::<Counter<_>>()
        .most_common_ordered();

    let top_five = common_words[0..5].to_vec();

    for (word, num) in &top_five {
        println!("Word '{}' has been found {} times ", word, num);
    }
}

struct Employee<'a> {
    name: &'a str,
    salary: f32,
    id: u32,
}