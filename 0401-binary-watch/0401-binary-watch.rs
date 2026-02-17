impl Solution {
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
        if turned_on == 0 {
            return vec!["0:00".to_string()];
        } else if turned_on == 9 {
            return vec![];
        }

        let turned_on = turned_on as usize;
        let mut results: Vec<String> = Vec::new();
        let mut hours: Vec<Vec<u8>> = vec![Vec::new(); turned_on.max(5) + 1];


        for hour in 0 ..= 11 {
            let hour = hour as u8;
            
            hours[hour.count_ones() as usize].push(hour);
        }

        for time in 0 ..= 59 {
            let time = time as usize;
            let time_ones = time.count_ones() as usize;

            if time_ones > turned_on {
                continue;
            }
            let key = turned_on - time_ones;

            for &hour in hours[key].iter() {
                results.push(format!("{hour}:{:02}", time));
            }
        }

        results
    }
}