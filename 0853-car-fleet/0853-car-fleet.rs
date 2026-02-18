impl Solution {
    pub fn car_fleet(
        target: i32, 
        position: Vec<i32>, 
        speed: Vec<i32>
    ) -> i32 {
        let mut fleet_count = 0;
        let mut cars: Vec<(i32, f32)> = position.into_iter()
            .zip(speed.into_iter())
            .map(|(p, s)| {
                (p, (target - p) as f32 / s as f32)
            })
            .collect();

        cars.sort_by_key(|&(p, _)| p);

        for car_id in (0 .. cars.len() - 1).rev() {
            if cars[car_id].1 > cars[car_id + 1].1 {
                fleet_count += 1;
            } else {
                cars[car_id] = cars[car_id + 1];
            }
        }

        fleet_count + 1
    }
}