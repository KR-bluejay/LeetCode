use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Eq, PartialEq, Debug)]
struct BlockTuple(i32, usize, usize);

impl Ord for BlockTuple {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.cmp(&self.0)
    }
}

impl PartialOrd for BlockTuple {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
        if height_map.is_empty() || height_map[0].is_empty() { return 0; }

        let m = height_map.len();
        let n = height_map[0].len();
        let mut water_amount = 0;

        let mut heap: BinaryHeap<BlockTuple> = BinaryHeap::with_capacity(m * n);
        let mut visited = vec![vec![false; n]; m];

        for row in 0..m {
            visited[row][0] = true;
            visited[row][n-1] = true;
            heap.push(BlockTuple(height_map[row][0], row, 0));
            heap.push(BlockTuple(height_map[row][n-1], row, n-1));
        }
        for col in 1..n-1 {
            visited[0][col] = true;
            visited[m-1][col] = true;
            heap.push(BlockTuple(height_map[0][col], 0, col));
            heap.push(BlockTuple(height_map[m-1][col], m-1, col));
        }

        let directions: [(i32, i32); 4] = [(-1,0),(1,0),(0,-1),(0,1)];

        while let Some(BlockTuple(height, row, col)) = heap.pop() {
            for &(dr, dc) in &directions {
                let nr = row as i32 + dr;
                let nc = col as i32 + dc;

                if nr < 0 || nr >= m as i32 || nc < 0 || nc >= n as i32 { continue; }
                let nr = nr as usize;
                let nc = nc as usize;

                if visited[nr][nc] { continue; }

                visited[nr][nc] = true;
                let nh = height_map[nr][nc];
                water_amount += (height - nh).max(0);
                heap.push(BlockTuple(height.max(nh), nr, nc));
            }
        }

        water_amount
    }
}