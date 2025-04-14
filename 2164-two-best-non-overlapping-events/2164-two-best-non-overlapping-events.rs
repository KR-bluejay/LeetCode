use std::collections::BTreeMap;

impl Solution {
    pub fn max_two_events(events: Vec<Vec<i32>>) -> i32 {
        let mut start_map: BTreeMap<i32, i32> = BTreeMap::new();
        let mut end_map: BTreeMap<i32, i32> = BTreeMap::new();

        let mut first_start_time = i32::MAX;
        let mut last_end_time = 0;



        for event_item in events.iter() {
            let start_time = event_item[0];
            let end_time = event_item[1];
            let event_value = event_item[2];

            first_start_time = first_start_time.min(start_time);
            last_end_time = last_end_time.max(end_time);

            start_map.entry(start_time)
                .and_modify(|v| {
                    if event_value > *v {
                        *v = event_value;
                    }
                })
                .or_insert(event_value);
            end_map.entry(end_time)
                .and_modify(|v| {
                    if event_value > *v {
                        *v = event_value;
                    }
                })
                .or_insert(event_value);
        }

        let mut best_reward = 0;
        let mut prev_reward = 0;

        for (start_time, reward) in start_map.iter_mut().rev() {
            prev_reward = prev_reward.max(*reward);

            *reward = prev_reward.clone();
        }

        prev_reward = 0;

        for (end_time, reward) in end_map.iter_mut() {
            prev_reward = prev_reward.max(*reward);

            *reward = prev_reward.clone();
        }

        // for (start_time, &reward) in start_map.iter() {
        //     let (end_reward, _) = end_map.range(start_time + 1..).next().unwrap_or((&0, &0));

        //     best_reward = best_reward.max(reward + end_reward);
        // }

        for (end_time, &end_reward) in end_map.iter() {
            let (_, start_reward) = start_map.range(end_time + 1 .. ).next().unwrap_or((&0, &0));

            best_reward = best_reward.max(start_reward + end_reward);
        }

        best_reward
    }
}