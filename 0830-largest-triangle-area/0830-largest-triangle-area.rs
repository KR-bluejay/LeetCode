impl Solution {
    pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
        let mut area: f64 = 0.0;

        for i in 0 .. points.len() - 2 {
            let ax = points[i][0];
            let ay = points[i][1];

            for j in i + 1 .. points.len() - 1 {
                let bx = (points[j][0] - ax);
                let by = (points[j][1] - ay);

                for k in j + 1 .. points.len() {
                    let cx = (points[k][0] - ax);
                    let cy = (points[k][1] - ay);

                    area = area.max(((bx * cy) - (by * cx)).abs() as f64 * 0.5);
                }
            }
        }

        area
    }
}