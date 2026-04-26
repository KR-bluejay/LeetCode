#[derive(Debug)]
struct UnionFind {
    parents: Vec<usize>,
    sizes: Vec<usize>,
}

impl UnionFind {
    #[inline(always)]
    fn new(n: usize) -> Self {
        let parents = (0 .. n).collect();
        let sizes = vec![1; n];

        Self {
            parents,
            sizes,
        }
    }
    
    #[inline(always)]
    fn find(&mut self, id: usize) -> usize {
        if self.parents[id] != id {
            self.parents[id] = self.find(self.parents[id]);
        }

        self.parents[id]
    }

    #[inline(always)]
    fn union(&mut self, left: usize, right: usize) -> bool {
        let p_left = self.find(left);
        let p_right = self.find(right);

        if self.sizes[p_left] < self.sizes[p_right] {
            self.parents[p_left] = p_right;
            self.sizes[p_right] += self.sizes[p_left];
        } else {
            self.parents[p_right] = p_left;
            self.sizes[p_left] += self.sizes[p_right];
        }

        p_left == p_right
    }

}
impl Solution {
    #[inline(always)]
    pub fn contains_cycle(grid: Vec<Vec<char>>) -> bool {
        let mut union_find = UnionFind::new(grid.len() * grid[0].len());

        for row_id in 0 .. grid.len() {
            for col_id in 0 .. grid[0].len() {
                let cur_pos = row_id * grid[0].len() + col_id;
                let upper_pos = cur_pos - grid[0].len();
                let left_pos = cur_pos - 1;


                let is_upper_cycle = row_id > 0 
                    && grid[row_id][col_id] == grid[row_id - 1][col_id]
                    && union_find.union(cur_pos, upper_pos);
                
                let is_left_cycle = col_id > 0 
                    && grid[row_id][col_id] == grid[row_id][col_id - 1]
                    && union_find.union(cur_pos, left_pos);

                if is_upper_cycle || is_left_cycle {
                    return true;
                }
            }
        }

        false
    }
}