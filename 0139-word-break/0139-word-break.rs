impl Solution {
    fn dp(
        word_id: usize, 
        dict_id: usize, 
        s: &Vec<char>, 
        word_dict: &Vec<String>, 
        visited: &mut Vec<Vec<bool>>
    ) -> bool {
        if word_id == s.len() {
            return true;
        }

        if visited[word_id][dict_id] {
            return false;
        }

        visited[word_id][dict_id] = true;

        for (word_dict_id, word_dict_item) in word_dict.iter().enumerate() {
            if word_dict_item.len() >= s.len() - word_id + 1 {
                continue;
            }
            let cur_str: String = s[word_id .. word_id + word_dict_item.len()].iter().collect();

            if cur_str == *word_dict_item {
                if Self::dp(word_id + word_dict_item.len(), word_dict_id, &s, &word_dict, visited) {
                    return true;
                }
            }
        }
        false

    }
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut visited: Vec<Vec<bool>> = vec![vec![false; word_dict.len()];s.len()];
        let s: Vec<char> = s.chars().collect();

        for (word_dict_id, word_dict_item) in word_dict.iter().enumerate() {
            if word_dict_item.len() > s.len() {
                continue;
            }
            let cur_str: String = s[0 .. word_dict_item.len()].iter().collect();


            if cur_str != *word_dict_item {
                continue;
            }
            if Self::dp(word_dict_item.len(), word_dict_id, &s, &word_dict, &mut visited) {
                return true;
            }
        }
        false
    }
}