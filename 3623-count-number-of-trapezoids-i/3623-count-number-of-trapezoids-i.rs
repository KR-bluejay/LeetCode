use std::collections::HashMap;

impl Solution {
    pub fn count_trapezoids(points: Vec<Vec<i32>>) -> i32 {
        const MODULO: i64 = 1_000_000_007;


        let mut point_map: HashMap<i32, i64> = HashMap::with_capacity(points.len());
        let mut result: i64 = 0;
        let mut sum: i64 = 0;

        for point in points {
            point_map.entry(point[1]).and_modify(|v| * v += 1).or_insert(1);
        }

        for (_, count) in point_map.iter() {
            let edge = count * (count - 1) / 2;
            
            result += sum * edge;
            result %= MODULO;

            sum += edge;
        }

        result as i32
    }
}