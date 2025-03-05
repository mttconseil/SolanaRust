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

fn main() {
    let mut contacts: Vec<Contact> = Vec::new();

    loop {
        println!("\n=== MENU ===");
        println!("1. Ajouter un contact");
        println!("2. Afficher tous les contacts");
        println!("3. Quitter");

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
            }
        }
    }

}
