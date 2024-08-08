// En utilisant une table de hachage et des vecteurs,
// créez une interface textuelle pour permettre à un utilisateur d'ajouter des noms d'employés
// dans un département d'une entreprise.
// Par exemple, “Ajouter Sally au bureau d'études” ou “Ajouter Amir au service commercial”.

// Ensuite, donnez la possibilité à l'utilisateur de récupérer une liste de
// toutes les personnes dans un département,
// ou tout le monde dans l'entreprise trié par département,
// et classés dans l'ordre alphabétique dans tous les cas.

use std::io;
use std::collections::HashMap;

#[derive(Debug)]
struct Company {
    departments: HashMap<String, Vec<String>>,
    employees: Vec<String>,
}

impl Company {
    fn new() -> Self {
        Company {
            departments: HashMap::new(),
            employees: Vec::new()
        }
    }

    fn add_department(&mut self, department: String) {
        self.departments.entry(department).or_insert(Vec::new());
    }

    fn add_employee(&mut self, employee: String) {
        self.employees.push(employee);
    }

    fn show(&self, pretty: Option<bool>) {
        if pretty != None {
            println!("{:#?}", self);
        } else {
            println!("{:?}", self);
        }
    }
}

fn cli_read_line() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Échec de la lecture de l'entrée");

    input.trim().to_string()
}

fn cli_add_employee(company: &mut Company) {
    println!("Ajouter un employé");

    let employee = cli_read_line();

    if !employee.is_empty() {
        company.add_employee(employee.to_string());
        println!("Le nouvel employé {} a bien été ajouté.", employee);
    }

}

fn cli_show_employees(company: &Company) {
    println!("Voir les employés");
    println!("{:?}", company.employees);
}

fn cli_show_department_employees(company: &Company) {
    println!("Voir les employés d'un departement");
}

fn main() {
    println!("==============");
    println!("Mon entreprise");
    println!("==============");

    // initialisation de données par defaut

    let employees = [
        "Richard",
        "Léa",
        "Mathieu",
        "Anne",
        "Jean-Christophe",
        "Julie",
    ];

    let departments = [
        "service technique",
        "service commercial",
        "ressource humaine",
        "bureau d'étude"
    ];

    let mut company: Company = Company::new();

    for department in departments {
        company.add_department(department.to_string());
    }

    for employee in employees {
        company.add_employee(employee.to_string());
    }

    // menu

    loop {
        println!();
        println!("Choisissez une option:");
        println!("1. Ajouter un employé");
        println!("2. Afficher les employés d'un departement");
        println!("3. Afficher tout les employés.");
        println!("ENTER pour Quitter");

        let input = cli_read_line();

        match input.as_str() {
            "1" => cli_add_employee(&mut company),
            "2" => cli_show_department_employees(&company),
            "3" => cli_show_employees(&company),
            _   => break
        };
    }
}

