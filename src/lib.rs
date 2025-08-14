use rust_embed::Embed;

pub mod letterbox;

#[derive(Embed)]
#[folder = "dicts/"]
struct Dict;

pub fn load_words() -> Vec<Vec<u8>> {
    let data = Dict::get("lb.txt").unwrap().data;
    data.split(|b| *b == b'\n')
        .map(|line| line.strip_suffix(b"\r").unwrap_or(line).to_owned())
        .filter(|line| !line.is_empty())
        .collect()
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct CSet(u32);

impl CSet {
    pub fn new(word: &[u8]) -> CSet {
        let mut set = 0u32;
        for c in word {
            set |= 1 << (*c % 32);
        }
        CSet(set)
    }

    pub fn merge(mut self, word: &[u8]) -> CSet {
        for c in word {
            self.0 |= 1 << (*c % 32);
        }
        self
    }
}
