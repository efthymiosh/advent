use std::collections::BTreeMap;

#[derive(Debug, Default)]
struct TrieNode<T> {
    c: char,
    next: BTreeMap<T, TrieNode<T>>,
    marker: bool,
}

#[derive(Debug, Default)]
pub struct Trie<T> {
    root: TrieNode<T>,
}

impl<T> Trie<T>
where
    T: Default + Ord + Clone + Copy,
{
    pub fn new() -> Self {
        Trie {
            root: TrieNode::default(),
        }
    }

    pub fn insert(&mut self, slice: &[T]) {
        let mut cur = &mut self.root;

        for c in slice {
            cur = cur.next.entry(*c).or_default();
        }
        cur.marker = true;
    }

    pub fn contains(&self, slice: &[T]) -> bool {
        let mut cur = &self.root;

        for c in slice {
            match cur.next.get(&c) {
                Some(node) => cur = node,
                None => return false,
            }
        }

        cur.marker
    }

    pub fn match_all(&self, slice: &[T]) -> Vec<usize> {
        let mut cur = &self.root;
        let mut ret = vec![];

        for (idx, c) in slice.iter().enumerate() {
            match cur.next.get(&c) {
                Some(node) => cur = node,
                None => return ret,
            }
            if cur.marker {
                ret.push(idx);
            }
        }

        ret
    }
}
