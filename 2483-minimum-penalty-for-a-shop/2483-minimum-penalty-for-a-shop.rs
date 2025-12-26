impl Solution {
    pub fn best_closing_time(customers: String) -> i32 {
        let customers: Vec<u8> = customers.into_bytes();
        let mut cache: Vec<i32> = vec![0; customers.len() + 1];

        for id in (0 .. customers.len()).rev() {
            cache[id] = cache[id + 1] + (customers[id] == b'Y') as i32;
        }

        let mut n_count = 0;

        for id in 0 .. customers.len() {
            n_count += (customers[id] == b'N') as i32;

            cache[id + 1] += n_count
        }

        let mut penalty_count = customers.len() as i32;
        let mut penalty_id = 0;


        for id in 0 .. cache.len() {
            if cache[id] < penalty_count {
                penalty_count = cache[id];
                penalty_id = id;
            }
        }

        penalty_id as i32
    }
}