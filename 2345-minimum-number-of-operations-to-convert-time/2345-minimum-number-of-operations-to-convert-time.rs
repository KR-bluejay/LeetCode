use std::str::FromStr;

impl Solution {
    pub fn convert_time(current: String, correct: String) -> i32 {
        let current_minute = current.split_once(":").map(|(h, m)| {
            h.parse::<i32>().unwrap() * 60 + m.parse::<i32>().unwrap()
        }).unwrap();
        let correct_minute = correct.split_once(":").map(|(h, m)| {
            h.parse::<i32>().unwrap() * 60 + m.parse::<i32>().unwrap()
        }).unwrap();

        let mut diff_minute = correct_minute - current_minute;

        let times = [60, 15, 5, 1];

        let mut convert_count = 0;

        for &time in times.iter() {
            if time > diff_minute {
                continue;
            }

            convert_count += diff_minute / time;
            diff_minute %= time;
        }


        convert_count
    }
}