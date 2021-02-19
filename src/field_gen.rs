pub struct FieldGenerator {
    pub counter: usize,
    dimension: usize,
}

impl FieldGenerator {
    pub fn new(dimension: usize) -> FieldGenerator {
        FieldGenerator{counter: 0, dimension}
    }
}

impl Iterator for FieldGenerator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.counter == self.dimension * self.dimension {
            return None;
        }

        let x = self.counter / self.dimension;
        let y = self.counter % self.dimension;
        self.counter += 1;

        Some((x, y))
    }
}