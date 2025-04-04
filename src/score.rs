/// Calcule le score à partir de la différence avec l’objectif, de la force et du nombre de ratés.
pub fn calculate_score(diff: u32, force: u32, miss: u32) -> u32 {
    let base = match diff {
        0 => 100,
        1..=5 => 80,
        6..=10 => 60,
        11..=20 => 40,
        21..=50 => 20,
        _ => 0,
    };

    (base + force) / (miss + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_exact() {
        let score = calculate_score(0, 50, 0);
        assert_eq!(score, 150); // (100 + 50) / (0 + 1)
    }

    #[test]
    fn test_score_far() {
        let score = calculate_score(30, 50, 2);
        assert_eq!(score, (20 + 50) / 3); // (70 / 3) = 23
    }
}