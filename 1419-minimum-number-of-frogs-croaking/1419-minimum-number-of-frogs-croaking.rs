use std::collections::HashMap;

impl Solution {
    #[inline]
    fn find_id(croak_byte: u8) -> usize {
        match croak_byte {
            b'c' => 0,
            b'r' => 1,
            b'o' => 2,
            b'a' => 3,
            b'k' => 4,
            _ => usize::MAX
        }
    }
    pub fn min_number_of_frogs(croak_of_frogs: String) -> i32 {
        let mut croaks: Vec<u16> = vec![0; 5];

        let mut active_frog = 0;
        let mut total_frog = 0;


        for croak in croak_of_frogs.into_bytes() {
            let id = Self::find_id(croak);
            croaks[id] += 1;

            if 0 < id && croaks[id - 1] < croaks[id] {
                return -1;
            }


            match id {
                0 => {
                    active_frog += 1;
                    total_frog = total_frog.max(active_frog);
                },
                1 => {

                },
                2 => {

                },
                3 => {

                },
                4 => {
                    active_frog -= 1;
                }, 
                _ => {}
            }
        }

        if active_frog == 0 {
            total_frog
        } else {
            -1
        }
    }
}