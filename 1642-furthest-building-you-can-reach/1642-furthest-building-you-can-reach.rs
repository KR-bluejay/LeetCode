use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn furthest_building(
        heights: Vec<i32>, 
        bricks: i32, 
        ladders: i32
    ) -> i32 {
        let ladders = ladders as usize;
        
        let mut brick_count = 0;
        let mut ladder_count = 0;
        let mut height_heap: BinaryHeap<Reverse<i32>> 
            = BinaryHeap::with_capacity(heights.len());

        for id in 0 .. heights.len() - 1 {
            if heights[id] >= heights[id + 1] {
                continue;
            }

            let mut diff = heights[id + 1] - heights[id];
            
            brick_count += diff;
            
            if height_heap.len() < ladders {
                ladder_count += diff;
                height_heap.push(Reverse(diff));
            } else {
                if let Some(&Reverse(h)) = height_heap.peek() 
                && h < diff {
                    ladder_count -= h;
                    ladder_count += diff;

                    height_heap.pop();
                    height_heap.push(Reverse(diff));
                }
            }



            if brick_count - ladder_count > bricks {
                return id as i32;
            }
        }

        heights.len() as i32 - 1 
    }
}