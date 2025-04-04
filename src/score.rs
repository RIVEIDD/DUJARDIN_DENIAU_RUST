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
