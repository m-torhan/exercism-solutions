use std::collections::{HashSet, HashMap};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut ret = HashSet::<&'a str>::new();

    for p in possible_anagrams {
        if p.to_lowercase() == word.to_lowercase() {
            continue;
        }
        if p.len() != word.len() {
            continue;
        }
        let mut counts: HashMap<char, i32> = HashMap::new();
        for c in word.to_lowercase().chars() {
            if !counts.contains_key(&c) {
                counts.insert(c, 0);
            }
            counts.insert(c, counts.get(&c).expect("ERROR") + 1);
        }
        for c in p.to_lowercase().chars() {
            if !counts.contains_key(&c) {
                counts.insert(c, 0);
            }
            counts.insert(c, counts.get(&c).expect("ERROR") - 1);
        }
        if counts.iter().all(|x| *x.1 == 0) {
            ret.insert(p);
        }
    }
    return ret;
}
