impl Solution {
    pub fn earliest_finish_time(
        land_start_time: Vec<i32>, 
        land_duration: Vec<i32>, 
        water_start_time: Vec<i32>, 
        water_duration: Vec<i32>
    ) -> i32 {
        let lst = land_start_time.iter()
            .zip(land_duration.iter())
            .map(|(&s, &d)| s + d)
            .min()
            .unwrap();
        let land_min = water_start_time.iter()
            .zip(water_duration.iter())
            .map(|(&s, &d)| s.max(lst) + d)
            .min()
            .unwrap();
        let wst = water_start_time.iter().zip(water_duration.iter())
            .map(|(&s, &d)| s + d)
            .min()
            .unwrap();
        let water_min = land_start_time.iter()
            .zip(land_duration.iter())
            .map(|(&s, &d)| s.max(wst) + d)
            .min()
            .unwrap();

        land_min.min(water_min)
 
    }
}