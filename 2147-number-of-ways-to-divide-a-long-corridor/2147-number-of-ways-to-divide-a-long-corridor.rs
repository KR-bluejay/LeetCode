impl Solution {
    pub fn number_of_ways(corridor: String) -> i32 {
        const MODULO: usize = 1_000_000_007;

        let corridor_bytes: Vec<u8> = corridor.into_bytes();
        let mut seat_count = 0;
        let mut last_id = 0;
        let mut total_count = 1;

        for (id, &corridor_byte) in corridor_bytes.iter().enumerate() {
            if corridor_byte == b'S' {
                seat_count += 1;

                if seat_count % 2 == 1 {
                    if last_id != 0 {
                        // println!("{id} {last_id}");
                        total_count *= (id - last_id);
                        total_count %= MODULO;
                    }
                } else {
                    last_id = id;
                }
            }
        }
        // println!("B: {total_count}");
        

        if seat_count % 2 == 1 || seat_count == 0 {
            return 0;
        }

        total_count.max(1) as i32
    }
}