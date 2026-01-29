impl Solution {
    pub fn minimum_cost(
        source: String, 
        target: String, 
        original: Vec<char>, 
        changed: Vec<char>, 
        costs: Vec<i32>
    ) -> i64 {
        let source = source.into_bytes();
        let target = target.into_bytes();

        let mut text_cost: Vec<Vec<i64>> = vec![vec![i64::MAX / 2; 26]; 26];

        for id in 0 .. original.len() {
            let original_id = (original[id] as u8 - b'a') as usize;
            let changed_id = (changed[id] as u8 - b'a') as usize;
            let cost = costs[id] as i64;

            text_cost[original_id][changed_id] 
                = text_cost[original_id][changed_id].min(cost);
        }

        for k in 0 .. 26 {
            text_cost[k][k] = 0;

            for i in 0 .. 26 {
                if text_cost[i][k] == i64::MAX / 2 {
                    continue;
                }

                for j in 0 .. 26 {
                    if text_cost[k][j] == i64::MAX / 2 {
                        continue;
                    }

                    text_cost[i][j] = text_cost[i][j]
                        .min(text_cost[i][k] + text_cost[k][j]);
                }
            }
        }


        let mut result = 0;

        for id in 0 .. source.len() {
            let source_id = (source[id] - b'a') as usize;
            let target_id = (target[id] - b'a') as usize;

            if text_cost[source_id][target_id] == i64::MAX / 2 {
                return -1;
            }

            result += text_cost[source_id][target_id];
        }


        result
    }
}