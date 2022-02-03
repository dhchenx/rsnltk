use std::collections::btree_map::Entry::*;
use std::collections::BTreeMap;

///
///
/// Credit:  https://github.com/LazyEmpiricist/text_analysis/blob/main/src/lib.rs
///

#[cfg(test)]
mod tests {
    use crate::native::text::*;
    use std::collections::HashMap;

    #[test]
    fn test_text_if_english() {
        let text = "I like you!";
        println!("{}", similar_with_english(text))
    }

    #[test]
    fn test_trim_to_words() {
        let words = "(_test] {test2!=".to_string();
        let trimmed = trim_to_words(words);
        println!("{:?}", trimmed)
    }

    #[test]
    fn test_count_words() {
        let words = vec![
            "one".to_string(),
            "two".to_string(),
            "two".to_string(),
            "three".to_string(),
            "three".to_string(),
            "three".to_string(),
        ];
        let counted = count_words(&words);
        let mut words_map = HashMap::new();
        words_map.insert("one".to_string(), 1 as u32);
        words_map.insert("two".to_string(), 2 as u32);
        words_map.insert("three".to_string(), 3 as u32);
        println!("{:?}",words_map);
        assert_eq!(counted, words_map);
    }
}

use std::collections::HashMap;

/// Sort words in HashMap<Word, Frequency> according to frequency into Vec<String, u32>.
pub fn sort_map_to_vec(
    frequency: HashMap<String, u32>,
) -> std::vec::Vec<(std::string::String, u32)> {
    let mut vec_sorted: Vec<(String, u32)> = frequency.into_iter().collect();
    vec_sorted.sort_by(|a, b| b.1.cmp(&a.1));
    vec_sorted
}

/// Get mininum index and guarantee that index is alway >=0
pub fn get_index_min(index: &usize) -> usize {
    if *index as isize - 5 < 0 {
        //check if index -5 would result in negative number, return 0 in case
        0
    } else {
        //if index-5 > 0, return index-5
        index - 5
    }
}

// Get maximum index and garantee that index does not exeed total length of Vec
pub fn get_index_max(index: &usize, max_len: &usize) -> usize {
    if index + 5 > *max_len {
        *max_len as usize
    } else {
        index + 5
    }
}






///
///
/// 
pub fn count_words(words: &[String]) -> std::collections::HashMap<std::string::String, u32> {
    let mut frequency: HashMap<String, u32> = HashMap::new();
    for word in words {
        //ignore words constiting of only one char?
        //if word.len() > 1 {
        *frequency.entry(word.to_owned()).or_insert(0) += 1;
        //}
    }
    frequency
}

/// Uses the Bhattacharyya coefficient to determine if text is likely to be English.
///
/// Higher is better.
pub fn similar_with_english(text: &str) -> f64 {
    // count of the number of times a character occurs in the given text
    let mut count: BTreeMap<char, f64> = BTreeMap::new();
    for letter in text.chars() {
        // println!("k = {}",char::to_uppercase(letter).to_string());

        //let k=char::to_uppercase(letter)[0] as char;

        let k=letter;
        let k=letter.to_uppercase().collect::<Vec<_>>()[0];
        match count.entry(k) {
            Vacant(entry) => { entry.insert(1f64); },
            Occupied(mut entry) => *entry.get_mut() += 1f64,
        }
    }

    // total number of characters in the given text
    let total = text.len() as f64;

    // relative frequency of letters in the English language
    let mut english_frequencies: BTreeMap<char, f64> = BTreeMap::new();
    english_frequencies.insert('A', 0.0651738);
    english_frequencies.insert('B', 0.0124248);
    english_frequencies.insert('C', 0.0217339);
    english_frequencies.insert('D', 0.0349835);
    english_frequencies.insert('E', 0.1041442);
    english_frequencies.insert('F', 0.0197881);
    english_frequencies.insert('G', 0.0158610);
    english_frequencies.insert('H', 0.0492888);
    english_frequencies.insert('I', 0.0558094);
    english_frequencies.insert('J', 0.0009033);
    english_frequencies.insert('K', 0.0050529);
    english_frequencies.insert('L', 0.0331490);
    english_frequencies.insert('M', 0.0202124);
    english_frequencies.insert('N', 0.0564513);
    english_frequencies.insert('O', 0.0596302);
    english_frequencies.insert('P', 0.0137645);
    english_frequencies.insert('Q', 0.0008606);
    english_frequencies.insert('R', 0.0497563);
    english_frequencies.insert('S', 0.0515760);
    english_frequencies.insert('T', 0.0729357);
    english_frequencies.insert('U', 0.0225134);
    english_frequencies.insert('V', 0.0082903);
    english_frequencies.insert('W', 0.0171272);
    english_frequencies.insert('X', 0.0013692);
    english_frequencies.insert('Y', 0.0145984);
    english_frequencies.insert('Z', 0.0007836);
    english_frequencies.insert(' ', 0.1918182);

    // update the counts to be the relative frequency of letters in the given text
    // and then calculate the Bhattacharyya coefficient as our score
    let mut score = 0.0;
    for letter in english_frequencies.keys() {
        match count.entry(*letter) {
            Vacant(entry) => { entry.insert(0.0); },
            Occupied(mut entry) => *entry.get_mut() /= total,
        }
        let partition_overlap = count[&*letter] * english_frequencies[&*letter];
        score += partition_overlap.sqrt();
    }

    score
}



///
///
/// Credits: Splits String at whitespaces and removes chars like , or ?. Change the relevant line to remove or add chars from provided String.
///
pub fn trim_to_words(content: String) -> std::vec::Vec<std::string::String> {
    let content: Vec<String> = content
        .to_lowercase()
        .replace(&['-'][..], " ")
        //should 's be replaced?
        .replace("'s", "")
        .replace(
            &[
                '(', ')', ',', '\"', '.', ';', ':', '=', '[', ']', '{', '}', '-', '_', '/', '\'',
                '’', '?', '!', '“', '‘',
            ][..],
            "",
        )
        .split_whitespace()
        .map(String::from)
        .collect::<Vec<String>>();
    content
}