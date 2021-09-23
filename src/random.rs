pub struct Random(pub u64);

impl Random {
    pub fn next(&mut self) -> u64 {
        let next = self.0;

        self.0 ^= self.0 << 3;
        self.0 ^= self.0 >> 13;
        self.0 ^= self.0 << 37;

        next
    }

    pub fn choose<'a, T>(&mut self, list: &'a Vec<T>) -> Option<&'a T> {
        let next = self.next() as usize;
        let idx = next % list.len();

        list.get(idx)
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
