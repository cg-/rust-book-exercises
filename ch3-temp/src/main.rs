// Rust Bk Ch 3 Ex
// Convert temperatures between Fahrenheit and Celsius.
use std::io;

fn main() {
    println!("Enter a temperature:");
    let mut temp = String::new();

    io::stdin()
        .read_line(&mut temp)
        .expect("error reading line");

    let temp :f32 = temp.trim().parse().expect("please enter a number");

    println!("Select if this is in:");
    let options = ["Fahrenheit", "Celsius"];

    for i in 0..options.len(){
        println!("{}: {}",i+1,options[i]);
    }

    let mut selection = String::new();

    io::stdin()
        .read_line(&mut selection)
        .expect("error reading line");

    let selection :usize = selection.trim().parse().expect("please enter a number");

    match options[selection-1]{
        "Fahrenheit" => {
            println!("converting Fahrenheit to Celsius");

            println!("{}",(temp - 32 as f32) * (5 as f32/9 as f32));

        },
        "Celsius" => {
            println!("converting Celsius to Fahrenheit");

            println!("{}",(temp * (9 as f32 /5 as f32) + 32 as f32));

        },
        _ => {
            println!("error: selection not found")
        },
    }
}

