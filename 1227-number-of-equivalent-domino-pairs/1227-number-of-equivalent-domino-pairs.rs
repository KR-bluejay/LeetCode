use std::collections::{HashMap, BTreeSet};

impl Solution {
    pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
        let mut domino_count: usize = 0;
        let mut domino_map: HashMap<(i32, i32), BTreeSet<usize>> = HashMap::with_capacity(dominoes.len() * 2);

        for i in 0 .. dominoes.len() {
            let domino_a = dominoes[i][0];
            let domino_b = dominoes[i][1];

            domino_map.entry((domino_a, domino_b)).or_insert(BTreeSet::new()).insert(i);
        }

        for i in 0 .. dominoes.len() {
            let domino_a = dominoes[i][0];
            let domino_b = dominoes[i][1];

            if let Some(domino_set) = domino_map.get(&(domino_a, domino_b)) {
                domino_count += domino_set.range((i + 1)..).count();
            }
            if domino_a != domino_b {
                if let Some(domino_set) = domino_map.get(&(domino_b, domino_a)) {
                    domino_count += domino_set.range((i + 1)..).count();
                }
            }
        }

        domino_count as i32
    }
}