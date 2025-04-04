mod player;
mod game;
mod objective;
mod score;
mod input;
mod utils;

use clap::Parser;
use player::Player;
use game::start_game;

/// Les objectifs sont générés sous forme de paires lettre:nombre (ex: 'a': 45).
/// Le joueur doit appuyer sur la bonne touche au bon moment pour marquer des points.
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
