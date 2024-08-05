fn main() {
    let days: [&str; 12] = [
        "premier",
        "deuxième" ,
        "troisième",
        "quatrième",
        "cinquième",
        "sixième",
        "septième",
        "huitième",
        "neuvième",
        "dixième",
        "onzième",
        "douzième"
    ];

    let gifts: [&str; 12] = [
        "une seule perdrix",
        "deux tourtereaux",
        "trois beaux rameaux",
        "quatre canards volants",
        "cinq lapins courants",
        "six chiens bavants",
        "sept moulins à vent",
        "huit vaches à lait",
        "neuf boeufs très laids",
        "dix pigeons blancs",
        "onze plats d'argents",
        "douze coqs chantants"
    ];


    for day in 0..12 {
        println!();
        println!("Le {} jour de Noël, que m'auras-tu donné, m'amie ?", days[day]);

        for gift in (0..=day).rev() {
            if day > 0 {
                if gift == 0 {
                    print!(" et ");
                } else if gift != day {
                    print!(", ");
                }
            }
            print!("{}", gifts[gift]);
        }
        print!(".");
        println!();
    }
}

