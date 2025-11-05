use std::collections::{ BTreeSet, HashMap };
use std::cmp::Ordering;
use itertools::Itertools;

#[derive(Debug, Eq, PartialEq)]
struct Block {
    num: i32,
    count: i32,
}

impl Ord for Block {
    fn cmp(&self, other: &Self) -> Ordering {
        other.count.cmp(&self.count)
            .then_with(|| other.num.cmp(&self.num))
    }
}

impl PartialOrd for Block {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Solution {
    #[inline(always)]
    fn rebalance(top_set: &mut BTreeSet<Block>, rest_set: &mut BTreeSet<Block>, top_sum: &mut i64) {
        if rest_set.is_empty() {
            return;
        }

        while top_set.last().unwrap().cmp(rest_set.first().unwrap()) == Ordering::Greater {
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
        let mut results: Vec<i64> = Vec::with_capacity(nums.len() - k);
        let mut num_map: HashMap<i32, i32> = HashMap::with_capacity(nums.len());
        let mut top_set: BTreeSet<Block> = BTreeSet::new();
        let mut rest_set: BTreeSet<Block> = BTreeSet::new();
        let mut top_sum: i64 = 0;
        
        for &num in nums.iter().take(k - 1) {
            num_map.entry(num).and_modify(|v| *v += 1).or_insert(1);
        }

        for (id, (&k, &v)) in num_map.iter().enumerate() {
            if id < x {
                top_sum += k as i64 * v as i64;
                top_set.insert(Block {
                    num: k,
                    count: v as i32
                });
            } else {
                rest_set.insert(Block {
                   num: k,
                   count: v as i32
                });
            }
        }

        // Self::rebalance(&mut top_set, &mut rest_set, &mut top_sum);
        
        for (num_id, &num_val) in nums.iter().enumerate().skip(k - 1) {
            if k <= num_id {
                let prev_num_val = nums[num_id - k];

                if prev_num_val == num_val {
                    results.push(*results.last().unwrap());

                    continue;
                }

                num_map.entry(prev_num_val).and_modify(|v| {
                    if top_set.remove(&Block {
                        num: prev_num_val,
                        count: *v
                    }) {
                        top_sum -= prev_num_val as i64;
                        top_set.insert(Block {
                            num: prev_num_val,
                            count: *v - 1
                        });
                    } else if rest_set.remove(&Block {
                        num: prev_num_val,
                        count: *v
                    }) {
                        rest_set.insert(Block {
                            num: prev_num_val,
                            count: *v - 1
                        });
                    }
                    *v -= 1
                });
            }

            num_map.entry(num_val).and_modify(|v| {
                if top_set.remove(&Block {
                    num: num_val,
                    count: *v
                }) {
                    top_sum += num_val as i64;
                    top_set.insert(Block {
                        num: num_val,
                        count: *v + 1
                    });
                } else if rest_set.remove(&Block {
                    num: num_val,
                    count: *v
                }) {
                    rest_set.insert(Block {
                        num: num_val,
                        count: *v + 1
                    });
                }
                *v += 1
            }).or_insert_with(|| {
                if top_set.len() < x {
                    top_sum += num_val as i64;
                    top_set.insert(Block {
                        num: num_val,
                        count: 1
                    });
                } else {
                    rest_set.insert(Block {
                        num: num_val,
                        count: 1
                    });
                }
                1
            });

            Self::rebalance(&mut top_set, &mut rest_set, &mut top_sum);

            results.push(top_sum);
        }

        
        results
    }
}