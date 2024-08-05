// ne fonctionne qu'avec l'ascii
fn first_word(s: &String) -> &str {
    let octets = s.as_bytes();

    for (i, &element) in octets.iter().enumerate() {
        if element == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let mut s = String::from("Hello world !");
    let first_word = first_word(&s);

    // s.clear(); // error !

    println!("string: '{}'", s);
    println!("first_word: '{}'", first_word);

    s.clear();
}


