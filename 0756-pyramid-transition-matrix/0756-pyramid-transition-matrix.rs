use std::collections::HashMap;

impl Solution {
    fn find_pyramid(
        id: usize, 
        allow_map: &HashMap<(u8, u8), Vec<u8>>,
        blocks: &mut Vec<u8>,
        block_cache: &mut Vec<Vec<Vec<bool>>>,
        row_ids: &Vec<usize>,
    ) -> bool {
        // println!("{id}");
        if id + 1 == blocks.len() {
            return true;
        }
        
        // else if !block_cache[id][blocks[id - 1] as usize][blocks[id] as usize] {
        //     return false;
        // }

        let mut result = false;

        let temp_id = row_ids.partition_point(|&v| v <= id) - 1;
        // println!("A: {temp_id} {row_ids:?}");

        if row_ids[temp_id] == id {
            // println!("B: {id}");
            result |= Self::find_pyramid(id + 1, allow_map, blocks, block_cache, row_ids);
            block_cache[id][blocks[id - 1] as usize][blocks[id] as usize] = result;

            return result;
        }

        let offset = id - row_ids[temp_id] - 1;
        let next_col_id = row_ids[temp_id + 1] + offset;

        if let Some(allow_blocks) = allow_map.get(&(blocks[id - 1], blocks[id])) {
            for &allow_id in allow_blocks.iter() {
                if offset == 0 
                || (
                    allow_map.contains_key(&(blocks[next_col_id - 1], allow_id)) 
                // && block_cache[next_col_id][blocks[next_col_id - 1] as usize][allow_id as usize]
                ) {
                    // println!("{id} {temp_id} {next_col_id} {offset}");
                    blocks[next_col_id] = allow_id;

                    result |= Self::find_pyramid(
                        id + 1,
                        allow_map, 
                        blocks, 
                        block_cache, 
                        row_ids,
                    );

                    if result {
                        break;
                    }
                }
            }
        }

        block_cache[id][blocks[id - 1] as usize][blocks[id] as usize] = result;
        block_cache[id][blocks[id - 1] as usize][blocks[id] as usize]
    }
    pub fn pyramid_transition(bottom: String, allowed: Vec<String>) -> bool {
        let block_count = bottom.len() * (bottom.len() + 1) / 2;
        let mut blocks: Vec<u8> = vec![0; block_count];

        let mut row_ids: Vec<usize> = Vec::with_capacity(bottom.len());
        let mut block_cache: Vec<Vec<Vec<bool>>> = vec![vec![vec![true; 6]; 6]; block_count];

        let mut block_id = 0;
        let mut step_count = bottom.len();

        for (i, id) in bottom.into_bytes().into_iter().enumerate() {
            blocks[i] = (id - b'A');
        }
        

        while block_id < block_count {
            row_ids.push(block_id);
            block_id += step_count;
            step_count -= 1;
        }

        let mut allow_map: HashMap<(u8, u8), Vec<u8>> = HashMap::with_capacity(allowed.len());

        for allow_block in allowed.into_iter() {
            let allow_block = allow_block.into_bytes();

            let (ab1, ab2, ab3) = (allow_block[0] - b'A', allow_block[1] - b'A', allow_block[2] - b'A');

            allow_map.entry((ab1, ab2)).or_insert(Vec::with_capacity(10)).push(ab3);
        }



        Self::find_pyramid(
            1,
            &allow_map, 
            &mut blocks,
            &mut block_cache, 
            &row_ids
        )
    }
}