impl Solution {
    pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
        let colors: Vec<u8> = colors.into_bytes();
        let mut id: usize = 0;
        let mut total_time: i32 = 0;

        let mut same_color_times: Vec<i32> = Vec::with_capacity(needed_time.len());

        while id < colors.len() {
            same_color_times.clear();
            same_color_times.push(needed_time[id]);
            
            let mut next_id = id + 1;

            while next_id < colors.len() && colors[id] == colors[next_id] {
                same_color_times.push(needed_time[next_id]);
                next_id += 1;
            }

            if same_color_times.len() > 1 {
                same_color_times.sort();

                for same_color_time in same_color_times[0 .. same_color_times.len() - 1].iter() {
                    total_time += same_color_time;
                }
            }


            id = next_id;
        }

        total_time
    }
}