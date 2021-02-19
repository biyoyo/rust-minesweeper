pub struct Adjacent {
    pub coord: (i8, i8),
    offset: (i8, i8),
    dimension: i8,
}

impl Adjacent {
    pub fn new(dimension: usize, x: usize, y: usize) -> Adjacent {
        Adjacent {
            coord: (x as i8, y as i8),
            offset: (-1, -1),
            dimension: dimension as i8,
        }
    }
}

impl Iterator for Adjacent {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let mut adjacent;
        loop {
            adjacent = (self.coord.0 + self.offset.0, self.coord.1 + self.offset.1);
            if self.offset.0 < 1 && self.offset.1 <= 1 {
                self.offset.0 += 1;
                if self.offset == (0, 0) {
                    self.offset.0 += 1;
                }
            } else if self.offset.0 >= 1 && self.offset.1 <= 1 {
                self.offset.0 = -1;
                self.offset.1 += 1;
            } else {
                return None;
            }
            if adjacent.0 >= 0
                && adjacent.0 < self.dimension
                && adjacent.1 >= 0
                && adjacent.1 < self.dimension
            {
                break;
            }
        }
        Some((adjacent.0 as usize, adjacent.1 as usize))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tests() {
        let mut adj = Adjacent::new(4, 1, 1);
        assert_eq!(adj.next(), Some((0, 0)));
    }
}