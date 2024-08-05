use std::io;

fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 1.8 + 32.0
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) / 1.8
}

fn get_degree_input(unit: &str) -> f64 {
    println!();
    println!("Degré en {} :", unit);
    println!();

    let input  = read_line();
    let input: f64 = match input.parse() {
        Ok(num) => num,
        Err(_)  => {
            println!("Vous devez saisir un nombre valide.");
            std::process::exit(1);
        }
    };

    input
}

fn read_line() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Echec de la lecture de l'entrée utilisateur.");

    input.trim().to_string()
}

fn cli_celsius_to_fahrenheit() {
    let input:  f64 = get_degree_input("celsius");
    let result: f64 = celsius_to_fahrenheit(input);
    println!("{}°C = {}°F", input, result);
}

fn cli_fahrenheit_to_celsius() {
    let input:  f64 = get_degree_input("fahrenheit");
    let result: f64 = fahrenheit_to_celsius(input);
    println!("{}°F = {}°C", input, result);
}

fn main() {
    println!("====================");
    println!("Celsius x Fahrenheit");
    println!("====================");

    println!();
    println!("Choisissez une conversion :");
    println!("1 : Celsius vers Fahrenheit");
    println!("2 : Fahrenheit vers Celsius");
    println!();

    let input = read_line();

    match input.as_str() {
        "1" => cli_celsius_to_fahrenheit(),
        "2" => cli_fahrenheit_to_celsius(),
        _ => {
            println!("Choix invalide.");
            std::process::exit(1);
        }
    }
}

