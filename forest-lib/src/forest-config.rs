pub struct ForestConfig {
    pub seed: u64,
    pub width: usize,
    pub height: usize,
    pub months: u32,
}

impl ForestConfig {
    pub fn new(seed: u64, width: usize, height: usize, months: u32) -> Self {
        Self { seed, width, height, months }
    }
}
