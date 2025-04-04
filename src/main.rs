// use rand::prelude::*;
// use rand::distributions::Alphanumeric;

// fn main() {
//     println!("Hello, world!");

//     // Get an RNG:
//     let mut rng = rand::thread_rng();

//     // Try printing a random unicode code point (may panic on invalid chars)
//     println!("char: '{}'", rng.r#gen::<char>());

//     // Try printing a random alphanumeric value instead!
//     let c = rng.sample(Alphanumeric) as char;
//     println!("alpha: '{}'", c);

//     // Generate and shuffle a sequence:
//     let mut nums: Vec<i32> = (1..100).collect();
//     nums.shuffle(&mut rng);

//     // And take a random pick:
//     if let Some(pick) = nums.choose(&mut rng) {
//         println!("random pick: {}", pick);
//     }
// }


//! Point d'entrée principal du jeu de duel
//! Utilise clap pour récupérer les arguments et initialise les joueurs
//! Lance la boucle de jeu jusqu'à la victoire d'un joueur

mod player;
mod game;
mod objective;
mod score;
mod input;
mod utils;

use clap::Parser;
use player::Player;
use game::start_game;

/// Représente les arguments CLI pour configurer la partie
#[derive(Parser, Debug)]
#[command(name = "Jeu de Duel", about = "Un mini-jeu de duel en ligne de commande")]
struct Args {
    /// Nom du joueur 1
    #[arg(long, default_value = "Joueur1")]
    name1: String,

    /// Nom du joueur 2
    #[arg(long, default_value = "Joueur2")]
    name2: String,

    /// Valeur initiale de vitalité
    #[arg(long, default_value_t = 50)]
    vitality: u32,

    /// Nombre d’objectifs par tour
    #[arg(long, default_value_t = 5)]
    objectifs: usize,
}

fn main() {
    let args = Args::parse();

    println!("##### Démarrage de la partie #####");

    // Création des deux joueurs avec des valeurs par défaut
    let mut player1 = Player::new(&args.name1, args.vitality);
    let mut player2 = Player::new(&args.name2, args.vitality);

    // Lancement de la boucle de jeu principale
    start_game(&mut player1, &mut player2, args.objectifs);
}
