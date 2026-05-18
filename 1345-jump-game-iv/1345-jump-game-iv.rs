use std::collections::{VecDeque, HashMap};

impl Solution {
    pub fn min_jumps(steps: Vec<i32>) -> i32 {
        let mut history = vec![i32::MAX; steps.len()];
        let mut step_map: HashMap<i32, Vec<usize>> = HashMap::with_capacity(steps.len());
        let mut step_queue: VecDeque<(usize, i32)> = VecDeque::with_capacity(steps.len());

        
        step_queue.push_back((0, 0));
        history[0] = 0;

        for (id, &step) in steps.iter().enumerate() {
            step_map.entry(step)
                .or_insert(vec![])
                .push(id);
        }


        while let Some((id, count)) = step_queue.pop_front() {
            if history[id] < count {
                continue;
            }


            if id + 1 < steps.len() 
            && count + 1 < history[id + 1] {
                history[id + 1] = count + 1;

                step_queue.push_back((id + 1, count + 1));
            }

            if id > 0 
            && count + 1 < history[id - 1] {
                history[id - 1] = count + 1;

                step_queue.push_back((id - 1, count + 1));
            }

            if let Some(ids) = step_map.remove(&steps[id]) {
                for next_id in ids.into_iter() {
                    if count + 1 < history[next_id] {
                        history[next_id] = count + 1;

                        step_queue.push_back((next_id, count + 1));
                    }
                }
            }
        }

        *history.last().unwrap()
    }
}