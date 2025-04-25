use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let mut word_map = HashMap::new();
    for c in word.chars() {
        let c = c.to_lowercase().next().unwrap();
        if let Some(val) = word_map.get_mut(&c) {
            *val += 1;
        } else {
            word_map.insert(c, 1);
        }
    }
    let mut res = HashSet::new();
    for &anagram in possible_anagrams {
        if anagram.len() != word.len() {
            continue;
        }
        if anagram.to_lowercase() == word.to_lowercase() {
            continue;
        }
        let mut map = HashMap::new();
        for c in anagram.chars() {
            let c = c.to_lowercase().next().unwrap();
            if let Some(val) = map.get_mut(&c) {
                *val += 1;
            } else {
                map.insert(c, 1);
            }
        }
        if map == word_map {
            res.insert(anagram);
        }
    }
    res
}
