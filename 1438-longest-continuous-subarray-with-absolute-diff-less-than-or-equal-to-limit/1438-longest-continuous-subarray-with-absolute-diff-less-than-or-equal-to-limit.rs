use std::collections::{ BinaryHeap };
use std::cmp::Reverse;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
        let mut min_heap: BinaryHeap<Reverse<(i32, usize)>> 
            = BinaryHeap::with_capacity(nums.len());
        let mut max_heap: BinaryHeap<(i32, usize)> 
            = BinaryHeap::with_capacity(nums.len());

        let mut result = 0;
        let mut left_id = 0;

        for (right_id, &right_num) in nums.iter().enumerate() {
            min_heap.push(Reverse((right_num, right_id)));
            max_heap.push((right_num, right_id));

            while (max_heap.peek().unwrap().0) - (min_heap.peek().unwrap().0.0) > limit {
                left_id = max_heap.peek().unwrap().1.min(min_heap.peek().unwrap().0.1) + 1;
                
                while min_heap.peek().unwrap().0.1 < left_id {
                    min_heap.pop();
                }

                while max_heap.peek().unwrap().1 < left_id {
                    max_heap.pop();
                }
            }

            result = result.max(right_id - left_id + 1);
        }

        result as i32
    }
}