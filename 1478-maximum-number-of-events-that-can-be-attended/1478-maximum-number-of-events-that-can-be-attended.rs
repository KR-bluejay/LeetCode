use std::cmp::{Reverse};
use std::collections::{BinaryHeap};

impl Solution {
    pub fn max_events(mut events: Vec<Vec<i32>>) -> i32 {
        let mut cur_day: i32 = 1;
        let mut event_count: i32 = 0;
        let mut event_id: usize = 0;

        let mut min_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::with_capacity(events.len());

        events.sort_unstable_by_key(|e| e[0]);

        while event_id < events.len() || !min_heap.is_empty() {
            if min_heap.is_empty() {
                cur_day = events[event_id][0];
            }

            while event_id < events.len() && events[event_id][0] == cur_day {
                min_heap.push(Reverse(events[event_id][1]));
                event_id += 1;
            }

            while let Some(Reverse(event_end_day)) = min_heap.peek() {
                if cur_day <= *event_end_day {
                    break;
                }
                min_heap.pop();
            }

            if min_heap.pop().is_some() {
                event_count += 1;
            }
            cur_day += 1;
        }

        event_count
    }
}