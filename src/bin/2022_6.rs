use itertools::Itertools;

fn all_unique(s: &[u8]) -> bool {
    s.into_iter().unique().count() == s.len()
}

fn marker_position(s: &[u8], n: &usize) -> usize {
    for i in 0..s.len()-(n-1) {
        if all_unique(&s[i..i+n]) {
            return i+n;
        }
    }
    panic!("Could not find marker");
}

fn main () {
    let signal: &[u8] = include_str!("../../inputs/2022_6.txt").as_bytes();
    println!("part1: {}", marker_position(signal, &4));
    println!("part2: {}", marker_position(signal, &14));
}
