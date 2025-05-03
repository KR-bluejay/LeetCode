impl Solution {
    pub fn min_domino_rotations(
        tops: Vec<i32>, 
        bottoms: Vec<i32>
    ) -> i32 {
        let mut domino_val = tops[0];
        let mut top_rotate_count = 0;
        let mut bottom_rotate_count = 0;

        let mut min_rotate_count = i32::MAX; 

        for i in 0 .. tops.len() {
            if tops[i] != domino_val && bottoms[i] != domino_val {
                top_rotate_count = i32::MAX;
                bottom_rotate_count = i32::MAX;
                break;
            }

            if tops[i] != domino_val {
                top_rotate_count += 1;
            } else if bottoms[i] != domino_val {
                bottom_rotate_count += 1;
            }
        }
        min_rotate_count = min_rotate_count.min(top_rotate_count.min(bottom_rotate_count));
        
        domino_val = bottoms[0];
        top_rotate_count = 0;
        bottom_rotate_count = 0;

        for i in 0 .. tops.len() {
            if tops[i] != domino_val && bottoms[i] != domino_val {
                top_rotate_count = i32::MAX;
                bottom_rotate_count = i32::MAX;
                break;
            }

            if tops[i] != domino_val {
                top_rotate_count += 1;
            } else if bottoms[i] != domino_val {
                bottom_rotate_count += 1;
            }
        }

        min_rotate_count = min_rotate_count.min(top_rotate_count.min(bottom_rotate_count));

        return if min_rotate_count == i32::MAX {
            -1
        } else {
            min_rotate_count
        }
    }
}