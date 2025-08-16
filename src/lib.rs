use std::borrow::Cow;
use std::sync::LazyLock;

use rust_embed::Embed;

pub mod letterbox;

static DICT: LazyLock<&'static [u8]> =
    LazyLock::new(|| match Dict::get("lb.txt").unwrap().data {
        Cow::Borrowed(s) => s,
        Cow::Owned(_) => unreachable!(),
    });

#[derive(Embed)]
#[folder = "dicts/"]
struct Dict;

pub fn load_words() -> impl Iterator<Item = &'static [u8]> {
    let foo = DICT
        .split(|b| *b == b'\n')
        .map(|line| line.strip_suffix(b"\r").unwrap_or(line))
        .filter(|line| !line.is_empty());
    foo
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
