#[derive(Debug)]
pub struct Sequence {
    pattern: [u32; 16],
}

impl Sequence {
    pub fn new(pattern: [u32; 16]) -> Sequence {
        Sequence { pattern: pattern }
    }
}

/*
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
*/

impl<'a> IntoIterator for &'a Sequence {
    type Item = u32;
    type IntoIter = SequenceIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        SequenceIterator {
            sequence: self,
            index: 0,
        }
    }
}

pub struct SequenceIterator<'a> {
    sequence: &'a Sequence,
    index: usize,
}

impl<'a> Iterator for SequenceIterator<'a> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.index {
            0..=15 => self.sequence.pattern[self.index],
            _ => return None,
        };

        self.index += 1;
        Some(result)
    }
}
