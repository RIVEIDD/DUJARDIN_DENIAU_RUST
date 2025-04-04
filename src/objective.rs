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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_objectives_count() {
        let obj = generate_objectives(5);
        assert_eq!(obj.len(), 5);
    }

    #[test]
    fn test_generate_objectives_keys_are_letters() {
        let obj = generate_objectives(5);
        for key in obj.keys() {
            assert!(key.is_alphabetic());
        }
    }
}