use std::collections::{ BinaryHeap };
use std::cmp::Reverse;

impl Solution {
    pub fn find_all_people(
        n: i32, 
        meetings: Vec<Vec<i32>>, 
        first_person: i32
    ) -> Vec<i32> {
        let people_count = n as usize;
        let first_person = first_person as usize;

        let mut secret_heap: BinaryHeap<Reverse<(i32, usize)>> 
            = BinaryHeap::with_capacity(people_count);
        let mut meeting_graph: Vec<Vec<(i32, usize)>> = vec![Vec::with_capacity(5); people_count];
        let mut has_secret: Vec<bool> = vec![false; people_count];
        let mut secret_count = 0;

        for meeting in meetings {
            let src_id = meeting[0] as usize;
            let dst_id = meeting[1] as usize;
            let time = meeting[2];

            meeting_graph[src_id].push((time, dst_id));
            meeting_graph[dst_id].push((time, src_id));
        }

        secret_heap.push(Reverse((0, 0)));
        secret_heap.push(Reverse((0, first_person)));

        while let Some(Reverse((time, id))) = secret_heap.pop() {
            if secret_count == people_count {
                break;
            }
            
            if has_secret[id] {
                continue;
            }
            has_secret[id] = true;
            secret_count += 1;

            for &(dst_time, dst_id) in meeting_graph[id].iter() {
                if dst_time < time || has_secret[dst_id] {
                    continue;
                }

                secret_heap.push(Reverse((dst_time, dst_id)));
            }
        }

        has_secret.into_iter()
            .enumerate()
            .filter_map(|(id, has_secret)| has_secret.then(|| id as i32))
            .collect()
    }
}