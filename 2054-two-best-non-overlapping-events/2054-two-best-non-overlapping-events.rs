use std::collections::BTreeMap;

impl Solution {
    pub fn max_two_events(mut events: Vec<Vec<i32>>) -> i32 {
        let mut start_map: BTreeMap<i32, i32> = BTreeMap::new();
        let mut max_score = 0;
        let mut result = 0;
        // let mut end_map: BTreeMap<i32, i32> = BTreeMap::new();

        events.sort_by(|lhs, rhs| lhs[0].cmp(&rhs[0]).then(lhs[1].cmp(&rhs[1])));

        for event in events.iter().rev() {
            let start_time = event[0];
            // let end_time = event[1];
            
            max_score = max_score.max(event[2]);

            start_map.entry(start_time)
                .and_modify(|v| { (*v) = (*v).max(max_score)})
                .or_insert(max_score);
        }

        for event in events.iter() {
            let end_time = event[1];
            let mut first_score = event[2];


            if let Some((k, v)) = start_map.range((end_time + 1)..).next() {
                // println!("{end_time} {k} {v}");
                first_score += *v;
            }
            result = result.max(first_score);
        }

        result
    }
}