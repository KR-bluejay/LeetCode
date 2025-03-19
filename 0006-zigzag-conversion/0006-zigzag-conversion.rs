impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }

        let mut rows: Vec<String> = vec![String::new(); num_rows as usize];
        
        let mut cur_row = 0;
        let mut direction = -1;

        for s_item in s.chars() {
            rows[cur_row as usize].push(s_item);

            if cur_row == 0 || cur_row == num_rows - 1 {
                direction *= -1;
            }
            cur_row += direction;
        }

        rows.concat()
    }
}