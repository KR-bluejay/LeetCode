#[derive(Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Solution {
    pub fn number_of_pairs(mut points: Vec<Vec<i32>>) -> i32 {
        let mut pair_count: i32 = 0;
        

        let points: Vec<Point> = points.into_iter().map(|v| Point {
            x: v[0],
            y: v[1],
        })
        .collect();


        for first_id in 0 .. points.len() {
            for second_id in 0 .. points.len() {
                if first_id == second_id 
                || points[first_id].x > points[second_id].x 
                || points[first_id].y < points[second_id].y {
                    continue;
                }

                let first_x = points[first_id].x;
                let first_y = points[first_id].y;

                let second_x = points[second_id].x;
                let second_y = points[second_id].y;

                if !points.iter().enumerate().any(|(range_id, range_point)| {
                    first_id != range_id && second_id != range_id 
                    && first_x <= range_point.x && range_point.x <= second_x 
                    && first_y >= range_point.y && range_point.y >= second_y
                }) {
                    pair_count += 1;
                }
            }
        }
    



        pair_count
    }
}