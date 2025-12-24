impl Solution {
    pub fn minimum_boxes(mut apples: Vec<i32>, mut capacities: Vec<i32>) -> i32 {
        apples.sort_unstable_by(|left, right| right.cmp(&left));
        capacities.sort_unstable_by(|left, right| right.cmp(&left));

        let mut apple_id = 0;
        let mut box_id = 0;

        while apple_id < apples.len() {
            let mut apple_count = apples[apple_id];

            while 0 < apple_count {
                if capacities[box_id] == 0 {
                    box_id += 1;
                }

                let count = capacities[box_id].min(apple_count);

                capacities[box_id] -= count;
                apple_count -= count;
            }

            apple_id += 1;
        }

        box_id as i32 + 1
    }
}