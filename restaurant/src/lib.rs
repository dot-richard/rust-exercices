mod dining_room {
    mod reception {
        fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod service {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

pub fn eat_to_restaurant() {
    // chemin absolu
    crate::dining_room::reception::add_to_waitlist();

    // chemin relatif
    dining_room::reception::add_to_waitlist();
}
