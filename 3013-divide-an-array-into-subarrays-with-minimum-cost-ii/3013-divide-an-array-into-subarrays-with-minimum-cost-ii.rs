use std::collections::{ BinaryHeap, HashMap };
use std::cmp::Reverse;

impl Solution {
    pub fn minimum_cost(nums: Vec<i32>, k: i32, dist: i32) -> i64 {
        let dist = dist as usize;
        let k = k as usize - 1;

        let mut small_heap: BinaryHeap<i32> = BinaryHeap::with_capacity(dist);
        let mut large_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::with_capacity(dist);

        let mut removal_map: HashMap<i32, u16> = HashMap::with_capacity(nums.len());

        let mut cur_cost = 0;

        for id in 1 ..= dist + 1 {
            cur_cost += nums[id] as i64;

            small_heap.push(nums[id]);
        }

        for _ in k .. small_heap.len() {
            let small_num = small_heap.pop().unwrap();

            cur_cost -= small_num as i64;
            large_heap.push(Reverse(small_num));
        }


        let mut min_cost = cur_cost;


        for id in (dist + 2) .. nums.len() {
            let out_num = nums[id - dist - 1];
            let in_num = nums[id];

            let mut balance = 0;

            *removal_map.entry(out_num).or_insert(0) += 1;


            if let Some(&small_num) = small_heap.peek() 
            && out_num <= small_num {
                balance -= 1;
                cur_cost -= out_num as i64;
            } else {
                balance += 1;
            }


            if let Some(&small_num) = small_heap.peek() 
            && in_num <= small_num {
                small_heap.push(in_num);
                cur_cost += in_num as i64;
                balance += 1;
            } else {
                large_heap.push(Reverse(in_num));
                balance -= 1;
            }


            while let Some(&small_num) = small_heap.peek() 
            && let Some(removal_count) = removal_map.get_mut(&small_num) 
            && *removal_count > 0 {
                *removal_count -= 1;
                small_heap.pop();
            }

            while let Some(&Reverse(large_num)) = large_heap.peek() 
            && let Some(removal_count) = removal_map.get_mut(&large_num) 
            && *removal_count > 0 {
                *removal_count -= 1;
                large_heap.pop();
            }


            if balance < 0 {
                let large_num = large_heap.pop().unwrap().0;
                
                small_heap.push(large_num);
                
                cur_cost += large_num as i64;
            } else if balance > 0 {
                let small_num = small_heap.pop().unwrap();
                
                large_heap.push(Reverse(small_num));
                
                cur_cost -= small_num as i64;
            }

            min_cost = min_cost.min(cur_cost);
        }

        nums[0] as i64 + min_cost
    }
}