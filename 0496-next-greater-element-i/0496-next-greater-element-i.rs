use std::collections::{HashMap};
impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut elem_map: HashMap<i32, i32> = nums2.iter().map(|v| (*v, -1)).collect();
        let mut elem_stack: Vec<i32> = Vec::with_capacity(nums2.len());


        for num_item in nums2.iter() {
            while let Some(stack_num) = elem_stack.pop() {
                if stack_num < *num_item {
                    elem_map.entry(stack_num).and_modify(|v| *v = *num_item);
                } else {
                    elem_stack.push(stack_num);
                    break;
                }
            }
            elem_stack.push(*num_item);
        }

        nums1.iter().map(|v| elem_map.get(v).unwrap().clone()).collect()
    }
}