impl Solution {
    pub fn count_covered_buildings(n: i32, mut buildings: Vec<Vec<i32>>) -> i32 {
        buildings.sort_unstable_by(|lhs, rhs| lhs[0].cmp(&rhs[0]).then(lhs[1].cmp(&rhs[1])));

        let mut cover_counts: Vec<i32> = vec![0; buildings.len()];
        let mut one_cover: Vec<bool> = vec![false; n as usize + 1];
        let mut two_cover: Vec<bool> = vec![false; n as usize + 1];

        for (id, building) in buildings.iter().enumerate() {
            let (x_pos, y_pos) = (building[0] as usize, building[1] as usize);

            if one_cover[x_pos] {
                cover_counts[id] += 1;
            } else {
                one_cover[x_pos] = true;
            }

            if two_cover[y_pos] {
                cover_counts[id] += 1;
            } else {
                two_cover[y_pos] = true;
            }
        }

        one_cover.fill(false);
        two_cover.fill(false);

        // println!("{cover_counts:?}");
        

        for (id, building) in buildings.iter().enumerate().rev() {
            let (x_pos, y_pos) = (building[0] as usize, building[1] as usize);
            // println!("{id} {x_pos} {y_pos}");

            if one_cover[x_pos] {
                cover_counts[id] += 1;
            } else {
                one_cover[x_pos] = true;
            }

            if two_cover[y_pos] {
                cover_counts[id] += 1;
            } else {
                two_cover[y_pos] = true;
            }
        }

        // println!("{cover_counts:?}");

        cover_counts.into_iter().filter(|v| *v == 4).count() as i32
    }
}