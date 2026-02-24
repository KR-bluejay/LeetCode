use std::collections::{ BinaryHeap, HashMap };
use std::cmp::{ Ordering, Reverse };

impl Solution {
    #[inline]
    fn get_median(
        min_heap: &BinaryHeap<Reverse<i32>>, 
        max_heap: &BinaryHeap<i32>, 
        num_cap: usize
    ) -> f64 {
        let max_num = *max_heap.peek().unwrap() as f64;

        if num_cap % 2 == 1 {
            max_num
        } else {
            let min_num = min_heap.peek().unwrap().0 as f64;

            (min_num + max_num) / 2.0
        }
    }
    pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
        let num_cap = k as usize;

        let mut delayed_map: HashMap<i32, u16> = HashMap::with_capacity(nums.len());
        
        let mut min_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::with_capacity(num_cap);
        let mut max_heap: BinaryHeap<i32> = BinaryHeap::with_capacity(num_cap);

        let mut results: Vec<f64> = Vec::with_capacity(nums.len() - num_cap + 1);

        
        for id in 0 .. num_cap {
            max_heap.push(nums[id]);
        }

        for _ in 0 .. (num_cap / 2) {
            let max_num = max_heap.pop().unwrap();

            min_heap.push(Reverse(max_num));
        }

        results.push(Self::get_median(&min_heap, &max_heap, num_cap));

        for id in num_cap .. nums.len() {
            let delayed_num = nums[id - num_cap];
            let new_num = nums[id];
            let mut balance = 0;

            *delayed_map.entry(delayed_num).or_insert(0) += 1;

            if let Some(&max_num) = max_heap.peek() 
            && delayed_num <= max_num {
                balance -= 1;
            } else {
                balance += 1;
            }

            if let Some(&max_num) = max_heap.peek() 
            && new_num <= max_num {
                balance += 1;
                max_heap.push(new_num);
            } else {
                balance -= 1;
                min_heap.push(Reverse(new_num));
            }

            match balance.cmp(&0) {
                Ordering::Less => {
                    max_heap.push(min_heap.pop().unwrap().0);
                },
                Ordering::Equal => {},
                Ordering::Greater => {
                    min_heap.push(Reverse(max_heap.pop().unwrap()));
                },
            }

            while let Some(&max_num) = max_heap.peek() 
            && let Some(&delay_count) = delayed_map.get(&max_num) 
            && delay_count > 0 {
                *delayed_map.get_mut(&max_num).unwrap() -= 1;
                max_heap.pop();
            }

            while let Some(&Reverse(min_num)) = min_heap.peek() 
            && let Some(&delay_count) = delayed_map.get(&min_num) 
            && delay_count > 0 {
                *delayed_map.get_mut(&min_num).unwrap() -= 1;
                min_heap.pop();
            }

            results.push(Self::get_median(&min_heap, &max_heap, num_cap));
        }

        results
    }
}