use std::io;
use std::str::FromStr;

#[derive(PartialEq)]
enum ConversionMode {
    CtoF,
    FtoC,
}

fn main() {

    let mode = choose_conversion();

    loop {

        println!("Please select a temperature:");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");


        let number: i32 = match i32::from_str(input.trim()) {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a valid number");
                continue;
            }
        };

        if mode == ConversionMode::CtoF {
            let conversion = (number * 9 / 5) + 32;
            println!("{}° Celsius = {}° Fahrenheit", input.trim(), conversion);
            break;
        } else if mode == ConversionMode::FtoC {
            let conversion = (number - 32) * 5 / 9;
            println!("{}° Fahrenheit = {}° Celsius", input.trim(), conversion);
            break;
        }

    }
}

fn choose_conversion() -> ConversionMode {

    loop {
        println!("Choose the conversion:");
        println!("1. Celsius to Fahrenheit");
        println!("2. Fahrenheit to Celsius");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim().parse() {
            Ok(1) => return ConversionMode::CtoF,
            Ok(2) => return ConversionMode::FtoC,
            _ => {
                println!("Invalid choice. Please try again");
                continue;
            }
        }
    }
}