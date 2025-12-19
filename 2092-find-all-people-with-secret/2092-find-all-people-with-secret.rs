use std::collections::{BTreeMap, HashMap, VecDeque};

impl Solution {
    pub fn find_all_people(
        n: i32, 
        mut meetings: Vec<Vec<i32>>, 
        first_person: i32
    ) -> Vec<i32> {
        meetings.sort_by(|lhs, rhs| lhs[2].cmp(&rhs[2]));


            
        let people_count = n as usize;
        let first_person = first_person as usize;

        let mut secrets: Vec<bool> = vec![false; people_count];
        
        secrets[0] = true;
        secrets[first_person] = true;

        let mut visit: Vec<bool> = vec![false; people_count];
        let mut share_queue: VecDeque<usize> = VecDeque::with_capacity(people_count);

        let mut meeting_map: HashMap<usize, Vec<usize>> = HashMap::with_capacity(people_count);
        let mut prev_time = 0;

        for meeting in meetings {
            let x = meeting[0] as usize;
            let y = meeting[1] as usize;
            let time = meeting[2] as usize;

            if prev_time != time {
                for &id in meeting_map.keys() {
                    if secrets[id] {
                        share_queue.push_back(id);
                        visit[id] = true;
                    }
                }
    
                while let Some(id) = share_queue.pop_front() {
                    let meeting_people = meeting_map.get(&id).unwrap();
    
                    for &neighbor_id in meeting_people.iter() {
                        if secrets[neighbor_id] || visit[neighbor_id] {
                            continue;
                        }
    
                        secrets[neighbor_id] = true;
                        visit[neighbor_id] = true;
                        share_queue.push_back(neighbor_id);
                    }
                }
                meeting_map.clear();
            }
            
            meeting_map.entry(x)
                .or_insert(Vec::with_capacity(10))
                .push(y);
            meeting_map.entry(y)
                .or_insert(Vec::with_capacity(10))
                .push(x);

            prev_time = time;
        }

        

        for &id in meeting_map.keys() {
            if secrets[id] {
                share_queue.push_back(id);
                visit[id] = true;
            }
        }

        while let Some(id) = share_queue.pop_front() {
            let meeting_people = meeting_map.get(&id).unwrap();

            for &neighbor_id in meeting_people.iter() {
                if secrets[neighbor_id] || visit[neighbor_id] {
                    continue;
                }

                secrets[neighbor_id] = true;
                visit[neighbor_id] = true;
                share_queue.push_back(neighbor_id);
            }
        }





        secrets.into_iter().enumerate().filter_map(|(id, v)| {
            if v {
                Some(id as i32)
            } else {
                None
            }
        }).collect()
    }
}