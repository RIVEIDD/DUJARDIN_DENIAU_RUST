use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashMap;

/// Génère une HashMap d'objectifs aléatoires : lettre -> nombre [0..100]
pub fn generate_objectives(n: usize) -> HashMap<char, u32> {
    let alphabet: Vec<char> = ('a'..='z').collect();
    let mut rng = rand::thread_rng();
    let mut map = HashMap::new();

    for &letter in alphabet.choose_multiple(&mut rng, n) {
        let value = rng.gen_range(0..=100);
        map.insert(letter, value);
    }

    map
}
