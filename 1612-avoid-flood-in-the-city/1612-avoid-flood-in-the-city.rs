use std::collections::{ BinaryHeap, VecDeque, HashMap };

impl Solution {
    pub fn avoid_flood(rains: Vec<i32>) -> Vec<i32> {
        let mut results: Vec<i32> = vec![1; rains.len()];
        let mut chances: VecDeque<usize> = VecDeque::new();
        let mut lake_flood_map: HashMap<i32, i32> = HashMap::new();

        for (id, &rain) in rains.iter().enumerate() {
            if rain == 0 {
                chances.push_back(id);

                continue;
            }

            let mut cur_flood = lake_flood_map.entry(rain).or_insert(-1);

            results[id] = -1;

            if *cur_flood == -1 {
                *cur_flood = id as i32;
            } else {
                if chances.len() == 0 {
                    return vec![];
                }

                match chances.binary_search(&(*cur_flood as usize + 1)) {
                    Ok(chance_id) => {
                        if chance_id >= chances.len() {
                            return vec![];
                        }
                        let a_id = chances.remove(chance_id).unwrap();

                        results[a_id] = rain;
                        *cur_flood = id as i32;
                    },
                    Err(chance_id) => {
                        if chance_id >= chances.len() {
                            return vec![];
                        }

                        let a_id = chances.remove(chance_id).unwrap();

                        results[a_id] = rain;
                        *cur_flood = id as i32;
                    }
                }
            }
        }

        results
    }
}