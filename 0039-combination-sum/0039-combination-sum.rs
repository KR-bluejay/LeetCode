use std::collections::{ HashSet };

impl Solution {
    fn dp(cur: &mut Vec<i32>, cur_sum: i32, candidates: &[i32], res: &mut HashSet<Vec<i32>>, target: i32) {
        if cur_sum == target {
            res.insert(cur.clone());

            return;
        }


        for (index, &candidate) in candidates.iter().enumerate() {
            if cur_sum + candidate <= target {
                cur.push(candidate);
                
                Self::dp(cur, cur_sum + candidate, &candidates[index..candidates.len()], res, target);

                cur.pop();
            }
        }
    }
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut res: HashSet<Vec<i32>> = HashSet::new();


        Self::dp(&mut Vec::new(), 0, &candidates, &mut res, target);

        res.into_iter().collect()
    }
}