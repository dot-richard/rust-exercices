struct User {
    active: bool,
    name: String,
    email: String,
    nb_connections: u64,
}

fn create_user(name: String, email: String) -> User {
    User {
        active: true,
        name,
        email,
        nb_connections: 1
    }
}

fn display_user(user: &User) {
    println!();
    println!("user.email:           {}", user.email);
    println!("user.name:            {}", user.name);
    println!("user.active:          {}", user.active);
    println!("user.nb_connections:  {}", user.nb_connections);
    println!();
}

fn main() {
    println!("=====");
    println!("USERS");
    println!("=====");

    let user: User = create_user(
        String::from("richard"),
        String::from("richard@user.com")
    );

    display_user(&user);

    let user2: User = User {
        email: String::from("richard_2@user.com"),
        ..user
    };

    display_user(&user2);
}
