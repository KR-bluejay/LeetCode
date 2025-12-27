use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn most_booked(n: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
        // meetings.sort_by(|lhs, rhs| lhs[0].cmp(&rhs[0]));
        meetings.sort();

        let room_count = n as usize;

        let mut meeting_count: Vec<i32> = vec![0; room_count];

        let mut room_heap: BinaryHeap<Reverse<usize>> = BinaryHeap::with_capacity(room_count);

        for id in 0 .. room_count {
            room_heap.push(Reverse(id));
        }

        let mut end_time_heap: BinaryHeap<Reverse<(i64, usize)>> = BinaryHeap::with_capacity(room_count);

        for (id, meeting) in meetings.into_iter().enumerate() {
            let mut meeting_start_time = meeting[0] as i64;
            let mut meeting_end_time = meeting[1] as i64;
            
            while let Some(&Reverse((end_time, free_room_id))) = end_time_heap.peek() 
            && end_time <= meeting_start_time {
                room_heap.push(Reverse(free_room_id));
                
                end_time_heap.pop();
            }

            if let Some(Reverse(free_room_id)) = room_heap.pop() {
                meeting_count[free_room_id] += 1;
                
                end_time_heap.push(Reverse((meeting_end_time, free_room_id)));
            } else if let Some(Reverse((end_time, free_room_id))) = end_time_heap.pop() {
                meeting_count[free_room_id] += 1;
                meeting_end_time += (end_time - meeting_start_time).max(0);

                end_time_heap.push(Reverse((meeting_end_time, free_room_id)))
            }
        }

        let mut most_id = 0;
        let mut most_time = 0;



        for (id, time) in meeting_count.into_iter().enumerate() {
            if time > most_time {
                most_id = id;
                most_time = time;
            }
        }


        most_id as i32
    }
}