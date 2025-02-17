# 🎓 Alyra Rust

## **1. Installer Rust**

Pour installer Rust, utilisez `rustup`, l'outil officiel d'installation et de gestion des versions :

- [Télécharger rustup](https://www.rust-lang.org/fr/tools/install)

Les commandes suivantes devraient suffire pour l'installation :
- curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
- rustup update
- rustup default stable

- Assurez-vous que les versions suivantes sont installées :
  - **Rust** : 1.85.0
  - **Cargo** : 1.85.0

---

## **2. Vérifier l'installation**

Ouvrez un terminal et tapez :

```sh
rustc --version
cargo --version
```

Si les versions s'affichent, Rust est bien installé.

---

## **3. Installer Rustlings**

(déjà fait, c'est le répertoire rustlings dans le projet)
- [Récupérer Rustlings](https://github.com/rust-lang/rustlings)

---

## **4. Lancer Rustlings**

- Lancer la commande pour débuter les exercices :
```sh
rustlings
```

---

## **5. Utiliser Rustlings**

- Demander un indice si vous êtes bloqué ou souhaitez une explication "hint" : 
```sh
h
```

- Passer à l'exercice suivant "next" :
```sh
n
```

---

## **5. Créer un Nouveau Projet**

Pour créer un projet Rust avec Cargo :

```sh
cargo new mon_projet
cd mon_projet
```

Cela génère la structure de base d'un projet Rust.

---

## **6. Compiler et Exécuter un Projet**

### Compiler le projet
```sh
cargo build
```

### Exécuter le projet
```sh
cargo run
```

---

## **7. Compiler un Fichier Unique sans Cargo**

Si vous souhaitez simplement compiler un fichier `main.rs` sans créer de projet :

```sh
rustc main.rs
```

Puis lancer l'éxécutable 

```sh
./main
```