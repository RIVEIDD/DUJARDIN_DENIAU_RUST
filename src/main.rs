use clap::{arg, Parser};
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
    let j1 = Joueur::new("joueur 1",10,0,4);
    j1.afficher_stats();
}
