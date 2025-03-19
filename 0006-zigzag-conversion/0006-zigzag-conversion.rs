impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let mut row_vec: Vec<String> = Vec::with_capacity(num_rows as usize);
        let mut s_iter = s.chars().peekable();

        row_vec.resize(num_rows as usize, String::new());

        loop {

            if s_iter.peek().is_none() {
                break;
            }
            
            for i in 0 .. num_rows {
                let opt_s_item = s_iter.next();

                if let Some(s_item) = opt_s_item {
                    row_vec[i as usize].push(s_item);
                } else {
                    break;
                }
            }

            for i in (1 .. num_rows - 1).rev() {
                let opt_s_item = s_iter.next();
                

                if let Some(s_item) = opt_s_item {
                    row_vec[i as usize].push(s_item);
                } else {
                    break;
                }
            }

        }

        row_vec.join("")
    }
}