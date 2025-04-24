impl Solution {
    fn dp(
        word1_id: usize, 
        word2_id: usize, 
        word1: &Vec<char>, 
        word2: &Vec<char>,
        dist_record: &mut Vec<Vec<i32>>
    ) -> i32 {
        if word2_id == word2.len() {
            return if word1_id == word1.len() {
                0
            } else {
                (word1.len() - word1_id) as i32
            };
        }

        if word1_id == word1.len() {
            return (word2.len() - word2_id) as i32;
        }

        if dist_record[word1_id][word2_id] != i32::MAX {
            return dist_record[word1_id][word2_id];
        }


        if word1[word1_id] == word2[word2_id] {
            dist_record[word1_id][word2_id] = Self::dp(
                word1_id + 1, 
                word2_id + 1, 
                word1, 
                word2, 
                dist_record
            );
            return dist_record[word1_id][word2_id];
        }

        let insert_cost = Self::dp(
            word1_id, 
            word2_id + 1, 
            word1, 
            word2, 
            dist_record
        ) + 1;



        let replace_cost = Self::dp(
            word1_id + 1, 
            word2_id + 1, 
            word1, 
            word2, 
            dist_record
        ) + 1;

        let delete_cost = Self::dp(
            word1_id + 1, 
            word2_id, 
            word1, 
            word2, 
            dist_record
        ) + 1;



        let min_cost = delete_cost.min(replace_cost).min(insert_cost);

        dist_record[word1_id][word2_id] = min_cost;
        dist_record[word1_id][word2_id]
    }
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let word1: Vec<char> = word1.chars().collect();
        let word2: Vec<char> = word2.chars().collect();

        let mut dist_record: Vec<Vec<i32>> = 
            vec![vec![i32::MAX; word2.len() + 10]; word1.len() + 10];

        if word1.len() == 0 {
            return word2.len() as i32;
        } else if word2.len() == 0 {
            return word1.len() as i32;
        }

        Self::dp(
            0, 
            0, 
            &word1, 
            &word2, 
            &mut dist_record
        )
    }
}