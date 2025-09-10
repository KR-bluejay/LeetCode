use std::collections::HashSet;

impl Solution {
    pub fn minimum_teachings(
        n: i32, 
        mut languages: Vec<Vec<i32>>, 
        friendships: Vec<Vec<i32>>
    ) -> i32 {
        let mut affected_user_set: HashSet<usize> = HashSet::with_capacity(languages.len());
        let user_langs: Vec<HashSet<i32>> = languages.into_iter()
            .map(|v| v.into_iter().collect())
            .collect();

        for friendship in &friendships {
            let first_id = friendship[0] as usize - 1;
            let second_id = friendship[1] as usize - 1;

            if user_langs[first_id].is_disjoint(&user_langs[second_id]) {
                affected_user_set.insert(first_id);
                affected_user_set.insert(second_id);
            }
        }

        if affected_user_set.is_empty() {
            return 0;
        }

        let mut user_known_list = vec![0usize; (n as usize) + 1];

        for &u in &affected_user_set {
            for &lang in &user_langs[u] {
                user_known_list[lang as usize] += 1;
            }
        }
        let best = user_known_list.into_iter().max().unwrap_or(0);

        (affected_user_set.len() - best) as i32

    }
}