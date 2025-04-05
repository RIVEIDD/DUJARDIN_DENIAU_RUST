/// Calcule la distance circulaire entre deux valeurs dans [0..100]
pub fn circular_diff(a: u32, b: u32) -> u32 {
    let direct = (a as i32 - b as i32).unsigned_abs();
    direct.min(100 - direct)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circular_diff_simple() {
        assert_eq!(circular_diff(10, 15), 5);
    }

    #[test]
    fn test_circular_diff_wraparound() {
        assert_eq!(circular_diff(95, 5), 10); // car on boucle à 100
    }
}
