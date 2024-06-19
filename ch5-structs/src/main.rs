// practice with structs
impl Person{
    fn create(first: String, last: String, age: u32) -> Person{
        Person{
            first,
            last,
            age,
        }
    }

    fn get_full(&self) -> String{
        format!("{} {}", self.first, self.last)
    }
}

#[derive(Debug)]
struct Person {
    first: String,
    last: String,
    age: u32,
}

fn main() {
    let to_make = [
        ("john", "black", 43),
        ("jack", "brown", 12),
        ("phil", "white", 87),
        ("adam", "blue", 52),
        ("eve", "green", 6),
    ];

    let mut list: Vec<Person> = vec![];

    for (first, last, age) in to_make.iter(){
        list.push(Person::create(first.to_string(), last.to_string(), *age));
    }

    for i in list.iter(){
        println!("{}", i.get_full());
    }
}
