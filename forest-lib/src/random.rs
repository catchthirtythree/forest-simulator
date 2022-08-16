pub struct Random {
    seed: u64,
}

impl Random {
    pub fn new(seed: u64) -> Self {
        Self { seed }
    }

    pub fn next(&mut self) -> u64 {
        let next = self.seed;

        self.seed ^= self.seed << 3;
        self.seed ^= self.seed >> 13;
        self.seed ^= self.seed << 37;

        next
    }

    pub fn choose<T: Copy>(&mut self, list: &Vec<T>) -> Option<T> {
        if !list.is_empty() {
            let next = self.next() as usize;
            let idx = next % list.len();
            Some(list[idx])
        } else {
            None
        }
    }

    pub fn shuffle<'a, T>(&mut self, list: &'a mut Vec<T>) -> &'a Vec<T> {
        for _ in 0..list.len() {
            let idx = self.next() as usize % list.len();
            let t = list.swap_remove(idx);
            list.push(t);
        }

        list
    }
}
