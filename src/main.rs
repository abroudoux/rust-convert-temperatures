fn main() {
    let celsius = temperature;
    let fahrenheit = celsius * 18 /10 + 32;
    println!("Fahrenheit: {}", fahrenheit)
}

fn choose_conversion() -> char {

    loop {
        println!("Choose the conversion:");
        println!("1. Celsius to Fahrenheit");
        println!("2. Fahrenheit to Celsius");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim().parse() {
            Ok(1) => return Ctof,
            Ok(2) => return FtoC,
            _ => {
                println!("Invalid choice. Please try again");
                continue;
            }
        }
    }
}