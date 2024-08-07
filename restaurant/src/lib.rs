mod dining_room;

pub use crate::dining_room::reception;

pub fn eat_to_restaurant() {
    // chemin absolu
    // crate::dining_room::reception::add_to_waitlist();
    // chemin relatif
    // dining_room::reception::add_to_waitlist();
    //let mut meal = kitchen::Breakfast::summer("fromages");
    //meal.toast = String::from("confiture");
    //println!("Je voudrais une tartine avec {} ", meal.toast);

    // La prochaine ligne ne va pas se compiler si nous ne la commentons pas,
    // car nous ne sommes pas autorisés à voir ou modifier le fruit de saison
    // qui accompagne le repas.
    // repas.fruit_de_saison = String::from("myrtilles");

    //let commande1 = kitchen::Appetizers::Soup;
    //let commande2 = kitchen::Appetizers::Salad;

    // grace à use ...
    reception::add_to_waitlist();
    reception::add_to_waitlist();
    reception::add_to_waitlist();
}
