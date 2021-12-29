use std::collections::{HashMap, HashSet};

fn main() {
    let a = "abcefcagbcefa";
    println!("{}", a);
    println!("{:?}", char_freq(a));
}

fn char_freq(word: &str) -> HashMap<usize,HashSet<char>> {
    // hashmap form will be [1: (a,b), 2: (f,e,c), ...]
    let mut char_freq = HashMap::new();
    for ch in word.chars() {
        let freq = word.matches(ch).count();
        let set = char_freq.entry(freq).or_insert(HashSet::new());
        set.insert(ch);
    }
    char_freq
}
