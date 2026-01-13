use std::cmp::Ordering;

impl Solution {
    pub fn separate_squares(mut squares: Vec<Vec<i32>>) -> f64 {
        let mut total_area: f64 = 0.0;

        let mut left_y = 0.0;
        let mut right_y = f64::MIN;


        for square in squares.iter() {
            let cur_area = (square[2] as f64) * (square[2] as f64);
            
            total_area += cur_area;

            let tmp = (square[1] + square[2]) as f64;
            
            // left_y = left_y.min(square[1] as f64);
            right_y = right_y.max(tmp);
        }

        while (1e-5) < (right_y - left_y).abs() {
            let mid_y = left_y + (right_y - left_y) / 2.0;
            let mut mid_area = 0.0;


            for id in 0 .. squares.len() {
                let (cur_x, cur_y, cur_len) 
                    = (squares[id][0] as f64, squares[id][1] as f64, squares[id][2] as f64);
                if mid_y > cur_y {
                    let height = (mid_y - cur_y).min(cur_len);
                    
                    mid_area += cur_len * height;
                }
            }


            if mid_area < (total_area / 2.0) {
                left_y = mid_y;
            } else {
                right_y = mid_y;
            }
        }



        right_y
    }
}