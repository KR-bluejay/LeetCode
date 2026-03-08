impl Solution {
    pub fn count_points(points: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut results: Vec<i32> = Vec::with_capacity(queries.len());
        
        for query in queries.iter() {
            let mut result = 0;
            let (qx, qy, qr) = (query[0], query[1], query[2]);
        
            for point in points.iter() {
                let (px, py) = (point[0], point[1]);
                let (dx, dy) = (qx - px, qy - py);
                
                result += (qr.pow(2) >= (dx.pow(2) + dy.pow(2))) as i32;
            }

            results.push(result);
        }

        results
    }
}