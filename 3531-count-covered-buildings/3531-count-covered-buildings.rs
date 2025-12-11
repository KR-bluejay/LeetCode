impl Solution {
    pub fn count_covered_buildings(n: i32, mut buildings: Vec<Vec<i32>>) -> i32 {
        buildings.sort_by(|lhs, rhs| lhs[0].cmp(&rhs[0]).then(lhs[1].cmp(&rhs[1])));

        let mut building_counts: Vec<i32> = vec![0; buildings.len()];
        let building_len = buildings.len() - 1;
        
        let mut x_cover: Vec<bool> = vec![false; n as usize + 1];
        let mut x_rev_cover: Vec<bool> = vec![false; n as usize + 1];
        
        let mut y_cover: Vec<bool> = vec![false; n as usize + 1];
        let mut y_rev_cover: Vec<bool> = vec![false; n as usize + 1];


        for id in 0 .. buildings.len() {
            let rev_id = building_len - id;

            let (x_pos, y_pos) = (buildings[id][0] as usize, buildings[id][1] as usize);
            let (x_rev_pos, y_rev_pos) = (buildings[rev_id][0] as usize, buildings[rev_id][1] as usize);

            if x_cover[x_pos] {
                building_counts[id] += 1;
            } else {
                x_cover[x_pos] = true;
            }

            if y_cover[y_pos] {
                building_counts[id] += 1;
            } else {
                y_cover[y_pos] = true;
            }

            if x_rev_cover[x_rev_pos] {
                building_counts[rev_id] += 1;
            } else {
                x_rev_cover[x_rev_pos] = true;
            }

            if y_rev_cover[y_rev_pos] {
                building_counts[rev_id] += 1;
            } else {
                y_rev_cover[y_rev_pos] = true;
            }

        }

        building_counts.into_iter().filter(|v| *v == 4).count() as i32
    }
}