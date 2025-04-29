use std::collections::{ BinaryHeap };
use std::cmp::{Ordering};

#[derive(Debug, Eq, PartialEq)]
struct NumItem {
    id: usize,
    val: i32,
}

impl Ord for NumItem {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.val == other.val {
            other.id.cmp(&self.id)
        } else {
            self.val.cmp(&other.val)
        }
    }
}

impl PartialOrd for NumItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        // Vector
        let k = k as usize;
        let mut p_queue = BinaryHeap::new();
        let mut max_count = 0;


        for (num_id, &num_val) in nums.iter().enumerate() {
            let max_num_item = p_queue.peek().unwrap_or(
                &NumItem {
                    id: 0,
                    val: 0
                }
            );

            if max_num_item.val < num_val {
                max_count = 0;
                p_queue.clear();
                p_queue.push(NumItem {
                    id: num_id,
                    val: num_val
                });
            } else if max_num_item.val == num_val {
                p_queue.push(NumItem {
                    id: num_id,
                    val: num_val
                });
            }

            if p_queue.len() > k {
                p_queue.pop();
            }
            
            let max_num_item = p_queue.peek().unwrap_or(
                &NumItem {
                    id: 0,
                    val: 0
                }
            );


            if p_queue.len() == k {
                max_count += max_num_item.id + 1;
            }
        }
        max_count as i64
    }
}