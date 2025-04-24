use std::collections::BinaryHeap;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut b_heap = BinaryHeap::from(nums);

        for i in 1 ..= k - 1 {
            b_heap.pop();
        }
        b_heap.pop().unwrap()
    }
}