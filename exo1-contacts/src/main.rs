<<<<<<< HEAD
use std::io::{self, Write};

/// Représente un contact avec un nom et un numéro de téléphone.
struct Contact {
    nom: String,
    telephone: String,
}

/// Affiche tous les contacts d'un vecteur.
fn afficher_contacts(contacts: &[Contact]) {
    if contacts.is_empty() {
        println!("Aucun contact à afficher.");
        return;
    }
    
    println!("\nListe des contacts :");
    for contact in contacts.iter() {
        println!("Nom: {}, Téléphone: {}", contact.nom, contact.telephone);
    }
}

/// Demande à l'utilisateur de saisir un contact et l'ajoute à la liste.
fn ajouter_contact(contacts: &mut Vec<Contact>) {
    println!("\nAjouter un nouveau contact");

    let nom = lire_entree("Nom : ");
    let telephone = lire_entree("Téléphone : ");

    contacts.push(Contact { nom, telephone });
    println!("✅ Contact ajouté avec succès !");
}

/// Fonction générique pour lire une entrée utilisateur.
fn lire_entree(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().expect("Erreur de flush");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erreur de lecture");
    
    input.trim().to_string()
}
=======
struct Contact {
    nom: String, 
    telephone: String
}

// méthode ajouter_contact
fn ajouter_contact(contacts: &mut Vec<Contact>){
    println!("Ajouter Contact");

    // nom contact
    println!("Tapez le nom du contact");
    let mut nom = String::new();
    std::io::stdin().read_line(&mut nom).expect("Erreur de lecture");

    // numero contact
    println!("Tapez le numéro de téléphone du contact");
    let mut numero = String::new();
    std::io::stdin().read_line(&mut numero).expect("Erreur de lecture");


    let new_contact = Contact {
        nom: nom,
        telephone: numero
    };
    contacts.push(new_contact);
}

// méthode afficher_contacts
fn afficher_contacts(contacts: &Vec<Contact>){
    for c in contacts {
        println!("Nom: {} Telephone: {}", c.nom, c.telephone);
    }
}   
>>>>>>> 3105b3d2c5f9f48356482465457e4f92681db8d1

fn main() {
    let mut contacts: Vec<Contact> = Vec::new();

    loop {
        println!("\n=== MENU ===");
        println!("1. Ajouter un contact");
        println!("2. Afficher tous les contacts");
        println!("3. Quitter");

<<<<<<< HEAD
        let choix = lire_entree("");

        //matching à faire

        match choix.as_str() {
            "1" => ajouter_contact(& mut contacts),
            "2" => {
                contacts.sort_by_key(|c| c.nom.clone());
                afficher_contacts(&contacts);
                println!("nombre de contacts : {}",contacts.len());
            }
                ,
            _ => {
                println!("Exit...");
                break;
=======
        let mut choix = String::new();
        std::io::stdin().read_line(&mut choix).expect("Erreur de lecture");
        let choix = choix.trim();

        // matching selon le choix, d'appeler la bonne méthode.
        match choix {
            "1" => ajouter_contact(&mut contacts),
            "2" => afficher_contacts(&contacts),
            _ => {
                println!("Echec")
>>>>>>> 3105b3d2c5f9f48356482465457e4f92681db8d1
            }
        }
    }

}
