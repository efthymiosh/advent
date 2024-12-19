use std::collections::{BTreeMap, BinaryHeap};

use nom::{
    bytes::complete::tag,
    character::complete::none_of,
    multi::{many1, separated_list0},
    sequence::separated_pair,
    IResult,
};
use util::{self, datastructs::Trie};

util::main![pt1, pt2];

fn parse_towels(input: &str) -> IResult<&str, (Vec<Vec<char>>, Vec<Vec<char>>)> {
    let (remainder, (towels, designs)) = separated_pair(
        separated_list0(tag(", "), many1(none_of(", \n"))),
        tag("\n\n"),
        separated_list0(tag("\n"), many1(none_of("\n"))),
    )(input)?;
    Ok((remainder, (towels, designs)))
}

pub fn pt1(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let (towels, designs) = util::parse::with_nom(&path, parse_towels)?;

    let mut trie: Trie<char> = Trie::new();
    for towel in &towels {
        trie.insert(&towel);
    }

    let mut possible = 0;
    for design in &designs {
        let mut heap = BinaryHeap::new();
        heap.push(0);
        while let Some(pos) = heap.pop() {
            if pos == design.len() {
                possible += 1;
                break;
            }
            for p in trie.match_all(&design[pos..]) {
                heap.push(pos + p + 1);
            }
        }
    }

    println!("Possible designs: {}", possible);

    Ok(())
}

pub fn solutions<'a>(trie: &Trie<char>, design: &'a[char], memo: &mut BTreeMap<&'a[char], usize>) -> usize {
    let mut ret = 0;
    if let Some(&x) = memo.get(design) {
        return x;
    }
    if design.is_empty() {
        return 1;
    }
    for p in trie.match_all(design) {
        let sols = solutions(trie, &design[(p + 1)..], memo);
        memo.insert(&design[(p+1)..], sols);
        ret += sols;
    }
    ret
}

pub fn pt2(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let (towels, designs) = util::parse::with_nom(&path, parse_towels)?;

    let mut trie: Trie<char> = Trie::new();
    for towel in &towels {
        trie.insert(&towel);
    }

    let mut possible = 0;
    let mut memo: BTreeMap<&[char], usize> = BTreeMap::new();
    for design in &designs {
        possible += solutions(&trie, design, &mut memo);
    }

    println!("Possible designs: {}", possible);

    Ok(())
}
