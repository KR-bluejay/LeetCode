use std::collections::{ BTreeSet, HashMap };
use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq)]
struct Block {
    num: i32,
    count: i32,
}

impl Ord for Block {
    fn cmp(&self, other: &Self) -> Ordering {
        // 1. 카운트가 높은 순 (내림차순)
        other.count.cmp(&self.count)
            // 2. 카운트가 같으면 값이 큰 순 (내림차순)
            .then_with(|| other.num.cmp(&self.num))
    }
}

impl PartialOrd for Block {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    #[inline(always)]
    fn rebalance(
        top_set: &mut BTreeSet<Block>, 
        rest_set: &mut BTreeSet<Block>, 
        top_sum: &mut i64, x: usize
    ) {
        while top_set.len() < x && !rest_set.is_empty() {
            let rest_item = rest_set.pop_first().unwrap();

            *top_sum += rest_item.num as i64 * rest_item.count as i64;

            top_set.insert(rest_item);
        }

        while !top_set.is_empty() && !rest_set.is_empty() {
            let worst_in_top = top_set.last().unwrap();
            let best_in_rest = rest_set.first().unwrap();

            if worst_in_top.cmp(best_in_rest) != Ordering::Greater {
                return;
            }

            let top_item = top_set.pop_last().unwrap();
            let rest_item = rest_set.pop_first().unwrap();

            *top_sum -= top_item.num as i64 * top_item.count as i64;
            *top_sum += rest_item.num as i64 * rest_item.count as i64;

            rest_set.insert(top_item);
            top_set.insert(rest_item);
        }
    }

    pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i64> {
        let k = k as usize;
        let x = x as usize;
        let mut results: Vec<i64> = Vec::with_capacity(nums.len() - k + 1);
        let mut num_map: HashMap<i32, i32> = HashMap::with_capacity(k);
        let mut top_set: BTreeSet<Block> = BTreeSet::new();
        let mut rest_set: BTreeSet<Block> = BTreeSet::new();
        let mut top_sum: i64 = 0;
        
        for &num in nums.iter().take(k - 1) {
            num_map.entry(num).and_modify(|v| *v += 1).or_insert(1);
        }

        for (&num, &count) in num_map.iter() {
            rest_set.insert(Block { num, count });
        }
        
        for (num_id, &num_val) in nums.iter().enumerate().skip(k - 1) {
            if num_id >= k {
                let prev_num_val = nums[num_id - k];

                if prev_num_val == num_val {
                    results.push(*results.last().unwrap());
                    
                    continue;
                }

                if let std::collections::hash_map::Entry::Occupied(mut entry) = num_map.entry(prev_num_val) {
                    let v = entry.get_mut();
                    let old_block = Block { num: prev_num_val, count: *v };
                    
                    if top_set.remove(&old_block) {
                        top_sum -= prev_num_val as i64; 
                        if *v > 1 {
                            top_set.insert(Block { num: prev_num_val, count: *v - 1 });
                        }
                    } else if rest_set.remove(&old_block) {
                         if *v > 1 {
                            rest_set.insert(Block { num: prev_num_val, count: *v - 1 });
                        }
                    }
                    
                    *v -= 1;
                    if *v == 0 {
                        entry.remove_entry();
                    }
                }
            }

            num_map.entry(num_val).and_modify(|v| {
                if top_set.remove(&Block { num: num_val, count: *v }) {
                    top_sum += num_val as i64;
                    top_set.insert(Block { num: num_val, count: *v + 1 });
                } else {
                    rest_set.remove(&Block { num: num_val, count: *v });
                    rest_set.insert(Block { num: num_val, count: *v + 1 });
                }
                *v += 1;
            }).or_insert_with(|| {
                rest_set.insert(Block { num: num_val, count: 1 });
                1
            });

            Self::rebalance(&mut top_set, &mut rest_set, &mut top_sum, x);
            results.push(top_sum);
        }
        
        results
    }
}