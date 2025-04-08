use std::collections::{HashSet};
impl Solution {
    fn backtrack(id: usize, tiles: &Vec<char>, temp: &mut String, visited: &mut Vec<bool>, cache: &mut HashSet<String>) {
        for i in 0 .. visited.len() {
            if visited[i] {
                continue;
            }

            temp.push(tiles[i]);

            if cache.insert(temp.clone()) {
                visited[i] = true;
                Self::backtrack(id + 1, tiles, temp, visited, cache);
                visited[i] = false;
            }
            temp.pop();
        }
    }
    pub fn num_tile_possibilities(mut tiles: String) -> i32 {
        let tiles: Vec<char> = tiles.chars().collect();
        let mut temp = String::with_capacity(tiles.len());
        let mut visited: Vec<bool> = vec![false; tiles.len()];
        let mut cache: HashSet<String> = HashSet::new();

        Self::backtrack(0, &tiles, &mut temp, &mut visited, &mut cache);

        cache.len() as i32
    }
}