// À partir d'une liste d'entiers,
// utiliser un vecteur et retourner la médiane (la valeur au milieu lorsque la liste est triée)
// et le mode (la valeur qui apparaît le plus souvent ;
// une table de hachage sera utile dans ce cas) de la liste.

use std::io;
use std::collections::HashMap;

fn mediane(numbers: &[i32]) -> f64 {
    let len = numbers.len();

    if len == 0 {
        return 0.0;
    }

    let mut sorted_numbers = numbers.to_vec();
    sorted_numbers.sort();

    if len % 2 == 0 {
        let mid1 = sorted_numbers[len / 2 - 1] as f64;
        let mid2 = sorted_numbers[len / 2] as f64;
        (mid1 + mid2) / 2.0
    } else {
        sorted_numbers[len / 2] as f64
    }
}

fn mode(numbers: &[i32]) -> i32 {
    let mut hash = HashMap::new();

    for &number in numbers {
        let count = hash.entry(number).or_insert(0);
        *count += 1;
    }

    let mut mode_number: i32 = 0;
    let mut max: i32 = 0;

    for (k, v) in hash {
        if max < v {
            max = v;
            mode_number = k;
        }
    }

    mode_number
}

fn main() {
    let mut vec: Vec<i32> = Vec::new();

    println!("Veuillez saisir des nombres.");
    println!("ENTER pour arrêter.");
    println!("Si aucun nombre, génération d'un vecteur par defaut.");

    loop {
        let mut input: String = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Erreur saisie utilisateur!");

        let input = input.trim();

        if input.is_empty() { break; }

        let number: i32 = input
            .parse()
            .expect("Veuillez saisir un nombre!");

        vec.push(number);
    }

    if vec.is_empty() {
        println!("Par defaut");
        vec = vec![1, 2, 3, 4, 5, 6, 6, 6, 7, 8, 9, 0];
    }

    println!("{:?}", &vec);
    println!("mediane:  {}", mediane(&vec));
    println!("mode:     {}", mode(&vec));
}
