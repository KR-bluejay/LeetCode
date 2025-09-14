use std::collections::HashMap;

impl Solution {
    pub fn spellchecker(wordlist: Vec<String>, queries: Vec<String>) -> Vec<String> {
        let mut word_map: HashMap<String, Vec<usize>> = HashMap::with_capacity(wordlist.len());
        let mut outputs: Vec<String> = Vec::with_capacity(queries.len());

        for (word_id, word) in wordlist.iter().enumerate() {
            let word: String = word.chars().filter_map(|v| {
                match v {
                    'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => None,
                    _ => Some(v.to_lowercase().next().unwrap())
                }
            }).collect();

            word_map.entry(word).or_insert(vec![]).push(word_id);
        }

        for (query_id, query) in queries.iter().enumerate() {
            let lower_query: String = query.chars().filter_map(|v| {
                match v {
                    'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => None,
                    _ => Some(v.to_lowercase().next().unwrap())
                }
            }).collect();

            if let Some(word_ids) = word_map.get(&lower_query) {
                let mut min_id = usize::MAX;
                let mut like_id = usize::MAX;

                for word_id in word_ids {
                    if query.len() != wordlist[*word_id].len() {
                        continue;
                    }

                    if *query == wordlist[*word_id] {
                        outputs.push(wordlist[*word_id].clone());
                        
                        break;
                    } else {
                        if query.to_lowercase() == wordlist[*word_id].to_lowercase() {
                            min_id = min_id.min(*word_id);
                        } else {
                            for (word_char, query_char) in query.chars().zip(wordlist[*word_id].chars()) {
                                if word_char.to_lowercase().next().unwrap() == query_char.to_lowercase().next().unwrap() {
                                    like_id = like_id.min(*word_id);

                                    continue;
                                }

                                match query_char {
                                    'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => {
                                        like_id = like_id.min(*word_id);
                                    },
                                    _ => {
                                        println!("{query_id} {word_char} {query_char}");
                                        println!("{:?} {:?}", query.chars(), wordlist[*word_id].chars());
                                        println!("");

                                        like_id = if like_id < *word_id { 
                                            like_id
                                        } else {
                                            usize::MAX
                                        };
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }

                if outputs.len() != query_id + 1 {
                    if min_id != usize::MAX {
                        outputs.push(wordlist[min_id].clone());
                    } else if like_id != usize::MAX {
                        outputs.push(wordlist[like_id].clone());
                    } else {
                        outputs.push("".to_string());
                    }
                }
            } else {
                outputs.push("".to_string());
            }
        }


        outputs
    }
}