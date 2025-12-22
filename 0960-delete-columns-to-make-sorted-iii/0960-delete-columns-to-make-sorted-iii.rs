impl Solution {
    fn find_size(
        str_bytes: &Vec<Vec<u8>>,
        id: usize,
        cache: &mut Vec<i32>,
    ) -> i32 {
        if cache[id] >= 0 {
            return cache[id];
        }

        let mut count = 1;

        for next_id in id + 1 .. str_bytes[0].len() {
            let mut is_sort = true;

            for row_id in 0 .. str_bytes.len() {
                is_sort &= str_bytes[row_id][id] <= str_bytes[row_id][next_id];

                if !is_sort {
                    break;
                }
            }
            if is_sort {
                count = count.max(Self::find_size(str_bytes, next_id, cache) + is_sort as i32);
            }
        }

        cache[id] = count;
        cache[id]
    }
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let str_bytes: Vec<Vec<u8>> = strs.into_iter().map(|v| v.into_bytes()).collect();
        let mut cache: Vec<i32> = vec![-1; str_bytes[0].len()];
        let mut result = 0;

        for id in 0 .. str_bytes[0].len() {
            result = result.max(Self::find_size(&str_bytes, id, &mut cache));
        }

        str_bytes[0].len() as i32 - result
    }
}