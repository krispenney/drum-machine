#[derive(Debug)]
pub struct Sequence {
    pattern: [u32; 16],
    current: usize
}

impl Sequence {
    pub fn new(pattern: [u32; 16]) -> Sequence {
        Sequence { pattern: pattern, current: 0 }
    }
}

impl Iterator for Sequence {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        match self.current {
            0..=15 => {
                let note = self.pattern[self.current];
                self.current += 1;
                Some(note)
            },
            _ => None,
        }
    }
}
