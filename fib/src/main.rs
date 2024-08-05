use std::io;

fn fib(n: i128) -> i128 {
    if n <= 2 {
        return 1
    } else {
        return fib(n - 1) + fib(n - 2)
    }
}

fn main() {
    println!("=======");
    println!("Fib nth");
    println!("=======");

    println!();
    println!("Générer le n-ième nombre de Fibonacci");
    println!();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Echec de la lecture de l'entrée utilisateur.");

    let input: i128 = input
        .trim()
        .parse()
        .expect("Veuillez saisir un nombre.");

    let result: i128 = fib(input);

    println!("{}", result);
}
