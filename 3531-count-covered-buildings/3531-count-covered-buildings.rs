impl Solution {
    pub fn count_covered_buildings(n: i32, mut buildings: Vec<Vec<i32>>) -> i32 {
        let city_count = n as usize + 1;

        let mut row_min = vec![usize::MAX; city_count];
        let mut row_max = vec![usize::MIN; city_count];

        let mut col_min = vec![usize::MAX; city_count];
        let mut col_max = vec![usize::MIN; city_count];

        for building in buildings.iter() {
            let (x_pos, y_pos) = (building[0] as usize, building[1] as usize);

            row_min[y_pos] = row_min[y_pos].min(x_pos);
            row_max[y_pos] = row_max[y_pos].max(x_pos);

            col_min[x_pos] = col_min[x_pos].min(y_pos);
            col_max[x_pos] = col_max[x_pos].max(y_pos);
        }

        buildings.into_iter().filter(|building| {
            let (x_pos, y_pos) = (building[0] as usize, building[1] as usize);

            row_min[y_pos] < x_pos && x_pos < row_max[y_pos] && col_min[x_pos] < y_pos && y_pos < col_max[x_pos]
        }).count() as i32
    }
}