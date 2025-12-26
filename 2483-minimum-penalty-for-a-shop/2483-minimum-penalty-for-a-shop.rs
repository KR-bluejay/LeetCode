impl Solution {
    pub fn best_closing_time(customers: String) -> i32 {
        let customers: Vec<u8> = customers.into_bytes();
        let mut min_penalty = 0;
        let mut min_hour = 0;

        let mut cur_penalty = 0;

        for (id, customer) in customers.into_iter().enumerate() {
            cur_penalty += (customer == b'N') as i32 - (customer == b'Y') as i32;

            if cur_penalty < min_penalty {
                min_hour = id + 1;
                min_penalty = cur_penalty;
            }
        }

        min_hour as i32
    }
}