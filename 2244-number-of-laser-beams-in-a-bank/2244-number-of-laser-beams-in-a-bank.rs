impl Solution {
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
        let mut bank_beam: Vec<i32> = Vec::with_capacity(bank.len());
        let mut total_count = 0;

        for (row_id, bank_row) in bank.into_iter().enumerate() {
            let mut row_count = bank_row.into_bytes().into_iter().filter(|v| *v == b'1').count();

            if row_count > 0 {
                bank_beam.push(row_count as i32);
            }
        }
    
        for (id, &item) in bank_beam.iter().enumerate().skip(1) {
            total_count += bank_beam[id - 1] * item;
        }

        total_count as i32
    }
}