use std::collections::HashMap;

impl Solution {
    fn dp(cur_str: &mut String, dial_map: &HashMap<u32, Vec<char>>, digit_nums: &Vec<u32>, ans: &mut Vec<String>,  pos: u32) {
        if pos as usize == digit_nums.len() {
            ans.push(cur_str.to_string());
            
            return;
        }

        
        let letter_list = dial_map.get(&digit_nums[pos as usize]).unwrap();

        for letter_item in letter_list {
            cur_str.push(*letter_item);

            Self::dp(cur_str, dial_map, digit_nums, ans, pos + 1);

            cur_str.pop();
        }
    }
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let dial_map: HashMap<u32, Vec<char>> = HashMap::from([
            (2, vec!['a', 'b', 'c']),
            (3, vec!['d', 'e', 'f']),
            (4, vec!['g', 'h', 'i']),
            (5, vec!['j', 'k', 'l']),
            (6, vec!['m', 'n', 'o']),
            (7, vec!['p', 'q', 'r', 's']),
            (8, vec!['t', 'u', 'v']),
            (9, vec!['w', 'x', 'y', 'z']),
        ]);

        let digit_nums: Vec<u32> = digits.chars().map(|c| c.to_digit(10).unwrap()).collect();
        let mut ans: Vec<String> = Vec::with_capacity(digit_nums.len().pow(4));

        if digit_nums.len() > 0 {
            Self::dp(&mut String::with_capacity(digit_nums.len()), &dial_map, &digit_nums, &mut ans, 0);
        }

        ans
    }
}