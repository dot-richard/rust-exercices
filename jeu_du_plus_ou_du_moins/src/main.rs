use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("==========================");
    println!("Le jeu du plus ou du moins");
    println!("==========================");

    // initialisation du jeu
    const MAX: u8 = 100;
    let mut tries: u8 = 10;
    let mut win: bool = false;
    let secret_number: u8 = rand::thread_rng().gen_range(1..=MAX);

    println!("");
    println!("Règles :");
    println!("  Vous devez deviner le nombre généré par l'ordinateur.");
    println!("  Le nombre généré est compris entre 1 et {}.", MAX);
    println!("");

    while tries > 0 {
        println!("Vous avez encore {} essais.", tries);

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Echec de la lecture de l'entrée utilisateur.");

        let input: u8 = match input.trim().parse() {
            Ok(number)  => number,
            Err(_)      => continue,
        };

        match input.cmp(&secret_number) {
            Ordering::Less      => println!("C'est plus."),
            Ordering::Greater   => println!("C'est moins."),
            Ordering::Equal     => {
                win = true;
                break;
            }
        }

        tries -= 1;
    }

    if win {
        println!("Vous avez gagné !");
    } else {
        println!("Vous avez perdu...");
        println!("Le nombre était {}.", secret_number);
    }
}

