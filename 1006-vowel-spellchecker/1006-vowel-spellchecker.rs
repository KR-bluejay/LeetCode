use std::collections::{ HashSet, HashMap };

impl Solution {
    pub fn spellchecker(wordlist: Vec<String>, queries: Vec<String>) -> Vec<String> {
        let mut spell_original_set: HashSet<String> = HashSet::with_capacity(wordlist.len());
        
        let mut spell_capital_map: HashMap<String, String> = HashMap::with_capacity(wordlist.len());
        let mut spell_vowel_map: HashMap<String, String> = HashMap::with_capacity(wordlist.len());
        let mut outputs: Vec<String> = Vec::with_capacity(queries.len());

        for word in wordlist.iter() {
            spell_original_set.insert(word.clone());
            spell_capital_map.entry(word.to_lowercase()).or_insert(word.clone());

            let vowel_word: String = word.chars().map(|v| {
                match v {
                    'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => '*',
                    _ => v.to_lowercase().next().unwrap()
                }
            }).collect();
            spell_vowel_map.entry(vowel_word).or_insert(word.clone());
        }

        for query in queries {
            if let Some(original_word) = spell_original_set.get(&query) {
                outputs.push(original_word.clone());
            } else if let Some(capital_word) = spell_capital_map.get(&query.to_lowercase()) {
                outputs.push(capital_word.clone());
            } else {
                let _vowel_word: String = query.chars().map(|v| {
                    match v {
                        'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => '*',
                        _ => v.to_lowercase().next().unwrap()
                    }
                }).collect();

                if let Some(vowel_word) = spell_vowel_map.get(&_vowel_word) {
                    outputs.push(vowel_word.clone());
                } else {
                    outputs.push("".to_string());
                }
            }
        }

        outputs
    }
}