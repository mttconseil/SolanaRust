fn main() {
    let mut contacts: Vec<Contact> = Vec::new();

    loop {
        println!("\n=== MENU ===");
        println!("1. Ajouter un contact");
        println!("2. Afficher tous les contacts");
        println!("3. Quitter");

        let mut choix = String::new();
        std::io::stdin().read_line(&mut choix).expect("Erreur de lecture");
        let choix = choix.trim();
    }
}
