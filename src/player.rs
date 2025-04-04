// //! Module représentant un joueur dans le jeu
// //! Contient les statistiques, méthodes d’affichage et de mise à jour
// use std::fmt;

// /// Structure représentant un joueur
// #[derive(Debug)]
// pub struct Player {
//     pub name: String,
//     pub vitality: i32,
//     pub speed: u64,    // en ms, utilisé comme délai entre les ticks
//     pub strength: u32, // bonus de score
// }

// impl Player {
//     /// Crée un nouveau joueur avec des valeurs par défaut
//     pub fn new(name: &str, vitality: u32) -> Self {
//         Self {
//             name: name.to_string(),
//             vitality: vitality as i32,
//             speed: 30,      // par défaut 30 ms
//             strength: 50,   // par défaut
//         }
//     }

//     /// Affiche les statistiques du joueur
//     pub fn display_stats(&self) {
//         println!(
//             "Au tour de {} (Vitality={}, Speed={}, Strength={})",
//             self.name, self.vitality, self.speed, self.strength
//         );
//     }

//     /// Applique un poison : réduction de speed ou de strength
//     pub fn apply_poison(&mut self, choice: u8) {
//         match choice {
//             1 => {
//                 if self.speed >= 10 {
//                     self.speed -= 5;
//                 }
//                 println!("{} perd 5 de vitesse !", self.name);
//             }
//             2 => {
//                 if self.strength >= 5 {
//                     self.strength -= 5;
//                 }
//                 println!("{} perd 5 de force !", self.name);
//             }
//             _ => println!("Choix invalide, aucun poison appliqué."),
//         }
//     }

//     /// Vérifie si le joueur est toujours en vie
//     pub fn is_alive(&self) -> bool {
//         self.vitality > 0
//     }
// }

// impl fmt::Display for Player {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(
//             f,
//             "{} (Vitality={}, Speed={}, Strength={})",
//             self.name, self.vitality, self.speed, self.strength
//         )
//     }
// }

//! Module représentant un joueur dans le jeu
//! Contient les statistiques, méthodes d’affichage et de mise à jour

/// Structure représentant un joueur avec son nom, vitalité, vitesse et force.
pub struct Player {
    pub name: String,
    pub vitality: i32,
    pub speed: u64,
    pub strength: u32,
}

impl Player {
    /// Crée un joueur avec un nom et une vitalité initiale.
    pub fn new(name: &str, vitality: u32) -> Self {
        Self {
            name: name.to_string(),
            vitality: vitality as i32,
            speed: 30,
            strength: 50,
        }
    }

    /// Affiche les caractéristiques du joueur dans le terminal.
    pub fn display_stats(&self) {
        println!("Au tour de {} (Vitality={}, Speed={}, Strength={})", self.name, self.vitality, self.speed, self.strength);
    }

    /// Applique un poison au joueur (réduit sa vitesse ou sa force).
    pub fn apply_poison(&mut self, choice: u8) {
        match choice {
            1 => {
                if self.speed >= 10 {
                    self.speed -= 5;
                }
                println!("{} perd 5 de vitesse !", self.name);
            }
            2 => {
                if self.strength >= 5 {
                    self.strength -= 5;
                }
                println!("{} perd 5 de force !", self.name);
            }
            _ => println!("Choix invalide, aucun poison appliqué."),
        }
    }

    pub fn is_alive(&self) -> bool {
        self.vitality > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_creation() {
        let player = Player::new("Alice", 50);
        assert_eq!(player.name, "Alice");
        assert_eq!(player.vitality, 50);
    }

    #[test]
    fn test_apply_poison_force() {
        let mut player = Player::new("Bob", 50);
        player.strength = 20;
        player.apply_poison(2); // Réduit la force
        assert_eq!(player.strength, 15);
    }
}