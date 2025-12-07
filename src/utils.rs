pub type Result<T> = miette::Result<T>;

pub trait Solution<T> {
    fn part1(input: &str) -> Result<T>;
    fn part2(input: &str) -> Result<T>;
}

#[allow(dead_code)]
pub trait Length {
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Length for u64 {
    fn len(&self) -> usize {
        let mut num = *self;
        let mut size = 0;
        while num > 0 {
            num /= 10;
            size += 1;
        }
        size
    }
}
