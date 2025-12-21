impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let row_bytes: Vec<Vec<u8>> = strs.into_iter().map(|v| v.into_bytes()).collect();
        let mut col_bytes: Vec<Vec<u8>> = vec![Vec::with_capacity(row_bytes.len()); row_bytes[0].len()];
        let mut result = 0;

        for row_id in 0 .. row_bytes[0].len() {
            for col_id in 0 .. row_bytes.len() {
                col_bytes[row_id].push(row_bytes[col_id][row_id]);
            }
        }


        let mut col_sort: Vec<bool> = vec![false; col_bytes[0].len()];

        for row_id in 0 .. col_bytes.len() {
            let mut col_id = 0;

            while col_id < col_bytes[0].len() - 1 {
                if !col_sort[col_id] && col_bytes[row_id][col_id] > col_bytes[row_id][col_id + 1] {
                    result += 1;
                    
                    break;
                }

                col_id += 1;
            }


            if col_id == col_bytes[0].len() - 1 {
                for col_id in 0 .. col_bytes[0].len() - 1 {
                    col_sort[col_id] |= col_bytes[row_id][col_id] < col_bytes[row_id][col_id + 1];
                }
            }
        }


        result
    }
}