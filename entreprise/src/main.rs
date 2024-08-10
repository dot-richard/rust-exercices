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

// retourner une entrée utilisateur
fn cli_read_line() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Échec de la lecture de l'entrée");

    input.trim().to_string()
}

fn cli_add_employee_to_department(
    departments: &mut HashMap<String, Vec<String>>,
    employees: &Vec<String>) {

    println!("------------------");
    println!("Ajouter un employé");

    // employés

    println!();
    println!("Saisissez un employé :");
    println!("ENTER pour arrêter.");
    for employee in employees {
        println!("{}", employee);
    }

    let employee = cli_read_line();

    if employee.is_empty() {
        println!("Prénom de l'employé vide.");
        return;
    }

    // departements

    println!();
    println!("Saisissez un departement :");
    println!("ENTER pour arrêter.");

    for key in departments.keys() {
        println!("{}", key);
    }

    let department = cli_read_line();

    if department.is_empty() {
        println!("Nom du département vide.");
        return;
    }

    // ajout

    if let Some(employees) = departments.get_mut(&department) {
        employees.push(employee);
        if let Some(employee) = employees.last() {
            println!(
                "Le nouvel employé '{}' a été ajouté au département '{}' avec succès.",
                employee, department
            );
        }
    } else {
        println!("Département '{}' inexistant.", department);
    }
}

fn cli_show_employees_for_department(departments: &HashMap<String, Vec<String>>) {
    println!("Saisissez un departement :");

    for dep in departments.keys() {
        println!("{}", dep);
    }

    let dep = cli_read_line();

    if let Some(employees) = departments.get(&dep) {
        cli_show_employees(&employees);
    } else {
        println!("Le departement '{}' est inexistant", dep);
    }
}

fn cli_show_all(departments: &HashMap<String, Vec<String>>) {
    for dep in departments.keys() {
        println!("{}", dep);
        cli_show_employees(&departments[dep])
    }
}

fn cli_show_employees(employees: &Vec<String>) {
    let mut employees = employees.clone();
    employees.sort();

    for emp in employees {
        println!("- {}", emp);
    }
}

fn main() {
    println!("==============");
    println!("Mon entreprise");
    println!("==============");

    // initialisation de données par defaut

    let data_departments: Vec<String> = vec![
        "service technique",
        "service commercial",
        "ressource humaine",
        "bureau d'étude"
    ].iter().map(|&s| s.to_string()).collect();

    let data_employees: Vec<String> = vec![
        "Richard",
        "Léa",
        "Mathieu",
        "Anne",
        "Jean-Christophe",
        "Julie",
    ].iter().map(|&s| s.to_string()).collect();

    let mut departments: HashMap<String, Vec<String>> = HashMap::new();

    for department in data_departments {
        departments.entry(department.to_string()).or_insert(Vec::new());
    }

    // menu

    loop {
        println!();
        println!("Choisissez une option :");
        println!("1. Ajouter un employé");
        println!("2. Afficher les employés d'un département");
        println!("3. Afficher tout les employés de tout les département");
        println!("ENTER pour Quitter");
        println!();

        let input = cli_read_line();

        match input.as_str() {
            "1" => cli_add_employee_to_department(&mut departments, &data_employees),
            "2" => cli_show_employees_for_department(&departments),
            "3" => cli_show_all(&departments),
            _   => break
        };
    }
}

