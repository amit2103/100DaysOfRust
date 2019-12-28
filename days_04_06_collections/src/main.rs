use counter::Counter;
use std::collections::HashMap;

fn main() {
    // 1 Tuple usage in Rust

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

    //3 .Using  Counters in rust, Rust counters are heavily inspired by Python Counters and
    // can be used by including the counter crate in the Cargo.toml file

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

    // JSON example in Rust
    // take a HashMap and convert it into a JSON

    json_example();
}

fn json_example () {
    let mut people = HashMap::new();
    people.insert("Amit".to_string(), "Coder".to_string());
    people.insert("Bob".to_string(), "Shooter".to_string());

    let json = serde_json::to_string(&people).unwrap();

    println!("Json for map is {} ", json);

    type Dictionary = HashMap<String, String>;
    let p : Dictionary  = serde_json::from_str(&json).unwrap();

    //Got it deserialized now lets try to see if the key 'Amit' has its associated value
    println!("Amit is a {} ", p.get("Amit").unwrap());


}

struct Employee<'a> {
    name: &'a str,
    salary: f32,
    id: u32,
}
