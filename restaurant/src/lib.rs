mod dining_room {
    pub mod reception {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    pub mod service {
        fn take_order() {}
        pub fn serve_order() {}
        fn take_payment() {}
    }
}

mod kitchen {
    pub enum Appetizers {
        Salad,
        Soup,
    }

    pub struct Breakfast {
        pub toast: String,
        fruits: String
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                fruits: String::from("peaches")
            }
        }
    }

    fn correct_incorrect_order() {
        cook_order();
        // On utilise le mot clé super pour appeler le module parent.
        // Ici, super se réfère au module dining_room
        // et super::serve_order() appelle la fonction serve_order définie dans ce module parent.
        super::dining_room::service::serve_order();
    }

    fn cook_order() {}
}

use crate::dining_room::reception;

pub fn eat_to_restaurant() {
    // chemin absolu
    // crate::dining_room::reception::add_to_waitlist();
    // chemin relatif
    // dining_room::reception::add_to_waitlist();
    let mut meal = kitchen::Breakfast::summer("fromages");
    meal.toast = String::from("confiture");
    println!("Je voudrais une tartine avec {} ", meal.toast);

    // La prochaine ligne ne va pas se compiler si nous ne la commentons pas,
    // car nous ne sommes pas autorisés à voir ou modifier le fruit de saison
    // qui accompagne le repas.
    // repas.fruit_de_saison = String::from("myrtilles");

    let commande1 = kitchen::Appetizers::Soup;
    let commande2 = kitchen::Appetizers::Salad;

    // grace à use ...
    reception::add_to_waitlist();
    reception::add_to_waitlist();
    reception::add_to_waitlist();
}


