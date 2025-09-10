use std::cmp::Ordering;
use std::collections::{ BTreeSet, HashSet};


impl Solution {
    pub fn minimum_teachings(
        lang_count: i32, 
        mut languages: Vec<Vec<i32>>, 
        mut friendships: Vec<Vec<i32>>
    ) -> i32 {
        let mut friendship_remove_set: BTreeSet<usize> = BTreeSet::new();
        let mut teaching_friend_set: BTreeSet<usize> = BTreeSet::new();
        let mut teaching_lang_set: BTreeSet<i32> = BTreeSet::new();

        for lang_id in 0 .. languages.len() {
            languages[lang_id].sort();
        }

        for (friendship_id, friendship) in friendships.iter().enumerate() {
            let left_id = friendship[0] as usize - 1;
            let right_id = friendship[1] as usize - 1;

            let left_langs = &languages[left_id];
            let right_langs = &languages[right_id];


            let mut left_lang_id: usize = 0;
            let mut right_lang_id: usize = 0;

            while left_lang_id < left_langs.len() && right_lang_id < right_langs.len() {
                match left_langs[left_lang_id].cmp(&right_langs[right_lang_id]) {
                    Ordering::Less => {
                        left_lang_id += 1;
                    }
                    Ordering::Equal => {
                        friendship_remove_set.insert(friendship_id);

                        break;
                    },
                    Ordering::Greater => {
                        right_lang_id += 1;
                    },
                }
            }
        }

        for (friend_id, friend_value) in friendships.into_iter().enumerate() {
            if friendship_remove_set.contains(&friend_id) {
                continue;
            }
            let friend_id_a = friend_value[0] as usize - 1;
            let friend_id_b = friend_value[1] as usize - 1;

            teaching_friend_set.insert(friend_id_a);
            teaching_friend_set.insert(friend_id_b);

            for lang_id in languages[friend_id_a].iter().chain(languages[friend_id_b].iter()) {
                teaching_lang_set.insert(*lang_id); 
            }
        }
        let mut min_teaching_count = i32::MAX;
        
        for lang_id in teaching_lang_set.iter() {
            let mut lang_teaching_count: i32 = 0;

            for friend_id in teaching_friend_set.iter() {
                if !languages[*friend_id].contains(lang_id) {
                    lang_teaching_count += 1;
                }
            }

            min_teaching_count = min_teaching_count.min(lang_teaching_count);
        }

        if min_teaching_count == i32::MAX {
            0
        } else {
            min_teaching_count
        }
    }
}