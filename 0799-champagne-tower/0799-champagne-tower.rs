impl Solution {
    pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
        let (query_row, query_col) = (query_row as usize, query_glass as usize);

        let total_count = (query_row + 2) * (query_row + 1) / 2;
        let mut champanes: Vec<f64> = vec![0.0; total_count];


        champanes[0] = poured as f64;

        println!("{}", champanes[0]);

        let mut floor_count = 1;
        let mut floor_max = 1;

        for id in 0 .. champanes.len() {
            if champanes[id] > 1.0 && id + floor_max < champanes.len() {
                let child_champane = (champanes[id] - 1.0) / 2.0;

                // println!("{id} {} {} {child_champane}", id + floor_max, id + floor_max + 1);
                
                champanes[id] = 1.0;
                champanes[id + floor_max] += child_champane;
                champanes[id + floor_max + 1] += child_champane;
            }

            if floor_max - 1 == query_row && floor_count - 1 == query_col {
                // println!("{id} {}", champanes[id]);

                return champanes[id].min(1.0);
            } else {
                // println!("FUCK: {floor_max} {query_row} {floor_count} {query_col}");
            }


            if floor_count == floor_max {
                floor_count = 1;
                floor_max += 1;
            } else {
                floor_count += 1;
            }
        }

        println!("{champanes:?}");

        0.0
    }
}