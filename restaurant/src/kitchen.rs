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
