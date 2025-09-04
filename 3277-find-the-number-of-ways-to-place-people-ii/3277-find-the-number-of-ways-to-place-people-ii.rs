use std::collections::{HashSet};

impl Solution {
    pub fn number_of_pairs(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_by(|point_a, point_b| {
            point_a[0].cmp(&point_b[0])
                .then_with(|| point_b[1].cmp(&point_a[1]))
        });
        
        let mut pair_count: i32 = 0;

        for alice_id in 0 .. points.len() - 1 {
            let (mut alice_x, mut alice_y) = (points[alice_id][0], points[alice_id][1]);
            let mut bob_y_min = i32::MIN;

            for bob_id in alice_id + 1 .. points.len() {
                let (bob_x, bob_y) = (points[bob_id][0], points[bob_id][1]);


                if alice_x <= bob_x && alice_y >= bob_y && bob_y_min <= bob_y {
                    alice_x = bob_x + 1;
                    bob_y_min = bob_y + 1;

                    pair_count += 1;
                }
            }
        }
    

        pair_count

    }
}