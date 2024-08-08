// Convertir des chaînes de caractères dans une variante du louchébem.

// La consonne initiale de chaque mot est remplacée par la lettre l
// et est rétablie à la fin du mot suivie du suffixe argotique “em” ;
// ainsi, “bonjour” devient “lonjourbem”.

// Si le mot commence par une voyelle, ajouter un l au début du mot
// et ajouter à la fin le suffixe “muche”.

// Et gardez en tête les détails à propos de l'encodage UTF-8 !

use std::io;

fn word_to_loucherbem(word: &str) -> String {
    let vowels = "aeiouyAEIOUY";

    // recuperer les chars
    let mut chars = word.chars();

    // recuperer la première lettre
    let first_letter = match chars.next() {
        Some(l) => l.to_string(),
        None    => return String::new(),
    };

    // recuperer le reste
    let rest = chars.collect::<String>();

    // faire en fonction de la première lettre
    if vowels.contains(&first_letter) {
        format!("l{}muche", rest)
    } else {
        format!("l{}{}em", rest, first_letter)
    }
}

fn loucherbem(string: &str) -> String {
    if string.is_empty() {
        return String::new();
    }

    let mut loucherbem_words: Vec<String> = Vec::new();

    for word in string.split_whitespace() {
        loucherbem_words.push(word_to_loucherbem(word));
    }

    loucherbem_words.join(" ")
}

fn main() {
    println!("==========");
    println!("Loucherbem");
    println!("==========");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Erreur avec l'entrée utilisateur.");

    let string = String::from(&input);

    println!("{}", loucherbem(&string));
}

