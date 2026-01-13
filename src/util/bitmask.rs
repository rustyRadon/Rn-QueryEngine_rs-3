pub struct BitMask {
    bits: Vec<u64>,
    pub length: usize,
}

impl BitMask {
    pub fn new(size: usize) -> Self {
        let num_holdable = (size + 63) / 64;
        Self {
            bits: vec![0u64; num_holdable],
            length: size,
        }
    }

    pub fn set(&mut self, index: usize) {
        let holdable_index = index / 64;
        let position = index % 64;
        self.bits[holdable_index] |= 1 << position;
    }

    pub fn unset(&mut self, index: usize) {
        let holdable_index = index / 64;
        let position = index % 64;
        self.bits[holdable_index] &= !(1 << position);
    }

    pub fn get(&self, index: usize) -> bool {
        let holdable_index = index / 64;
        let position = index % 64;
        (self.bits[holdable_index] & (1 << position)) != 0
    }

    pub fn count_active(&self) -> usize {
        self.bits.iter()
            .map(|chunk| chunk.count_ones() as usize)
            .sum()
    }
}