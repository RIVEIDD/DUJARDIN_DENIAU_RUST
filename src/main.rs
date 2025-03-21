use clap::{arg, Parser};
use rand::Rng;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long)]
    nom1: String,

    #[arg(long)]
    nom2: String,
}

struct Joueur {
    nom: String,
    vitalite: i32,
    vitesse: u64,
    force: u32
}

impl Joueur {
    fn new(new_name: &str, new_vitality: i32, new_speed: u64, new_strength: u32) -> Self { //Constructeur
        Joueur {
            nom: new_name.to_string(),
            vitalite: new_vitality,
            vitesse: new_speed,
            force: new_strength
        }
    }

    fn afficher_stats (&self) {
        println!("name: {} ",self.nom);
        println!("vitalite: {} ",self.vitalite);
        println!("vitesse: {} ",self.vitesse);
        println!("force: {} ",self.force);
    }
}

fn main() {
    println!("Hello, world!");

    let args = Args::parse();

    let mut rng = rand::thread_rng();

    let joueur1 = Joueur::new(
        &args.nom1,
        rng.gen_range(50..=100),
        rng.gen_range(30..=70),
        rng.gen_range(20..=60),
    );
    let joueur2 = Joueur::new(
        &args.nom2,
        rng.gen_range(50..=100),
        rng.gen_range(30..=70),
        rng.gen_range(20..=60),
    );

    println!("--- Joueur 1 ---");
    joueur1.afficher_stats();
    println!("--- Joueur 2 ---");
    joueur2.afficher_stats();
}
