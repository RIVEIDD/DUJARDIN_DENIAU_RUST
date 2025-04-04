/// Calcule la distance circulaire entre deux valeurs dans [0..100]
pub fn circular_diff(a: u32, b: u32) -> u32 {
    let direct = (a as i32 - b as i32).abs() as u32;
    direct.min(100 - direct)
}
