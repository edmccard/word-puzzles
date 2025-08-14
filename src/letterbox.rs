use itertools::Itertools;

use crate::CSet;

pub struct Puzzle {
    sides: [i8; 32],
    lset: CSet,
}

impl Puzzle {
    pub fn new(letters: &[u8]) -> Puzzle {
        let mut sides = [-1; 32];
        for i in 0..letters.len() {
            sides[(letters[i] as usize) % 32] = (i / 3) as i8;
        }
        Puzzle {
            sides,
            lset: CSet::new(letters),
        }
    }

    pub fn valid_word(&self, word: &[u8]) -> bool {
        word.iter()
            .try_fold(-1, |prev, c| {
                let side = self.sides[(*c as usize) % 32];
                (side >= 0 && side != prev).then_some(side)
            })
            .is_some()
    }

    pub fn solutions<'a>(
        &self,
        words: &'a [Vec<u8>],
    ) -> impl Iterator<Item = (&'a Vec<u8>, &'a Vec<u8>)> {
        words
            .iter()
            .filter(move |w| self.valid_word(w))
            .combinations(2)
            .filter_map(move |p| {
                let a = p[0];
                let b = p[1];
                if a[a.len() - 1] == b[0] {
                    (self.lset == CSet::new(a).merge(b)).then_some((a, b))
                } else if b[b.len() - 1] == a[0] {
                    (self.lset == CSet::new(a).merge(b)).then_some((b, a))
                } else {
                    None
                }
            })
    }
}
