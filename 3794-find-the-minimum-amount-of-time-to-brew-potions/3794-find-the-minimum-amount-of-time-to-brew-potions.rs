impl Solution {
    pub fn min_time(skill: Vec<i32>, mana: Vec<i32>) -> i64 {
        let mut offsets: Vec<i64> = vec![0; mana.len()];
        let mut times: Vec<Vec<i64>> = vec![vec![0; skill.len()]; mana.len()];
        
        let max_skill_id = skill.len() - 1;
        let max_mana_id = mana.len() - 1;

        for (mana_id, &mana_val) in mana.iter().enumerate() {
            for (skill_id, &skill_val) in skill.iter().enumerate() {
                let prev_brew_time = if skill_id == 0 {
                    0
                } else {
                    times[mana_id][skill_id - 1]
                };
                let cur_brew_time = (mana[mana_id] * skill_val) as i64;
    
                times[mana_id][skill_id] = prev_brew_time + cur_brew_time;
            }
        }

        for (mana_id, &mana_val) in mana.iter().enumerate().skip(1) {
            for (skill_id, &skill_val) in skill.iter().enumerate() {
                if skill_id == 0 {
                    offsets[mana_id] += (times[mana_id - 1][skill_id] + offsets[mana_id - 1]);
                }

                let cur_brew_time = offsets[mana_id] + times[mana_id][skill_id];
                let prev_brew_time = offsets[mana_id - 1] + if skill_id == max_skill_id {
                    0
                } else {
                    times[mana_id - 1][skill_id + 1]
                };
                let cur_offset = prev_brew_time - cur_brew_time;

                offsets[mana_id] += if cur_offset <= 0 {
                    0
                } else {
                    cur_offset
                };
            }
        }

        times[max_mana_id][max_skill_id] + offsets[max_mana_id]
    }
}