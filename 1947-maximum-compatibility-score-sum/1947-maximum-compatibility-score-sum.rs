impl Solution {
    fn get_score(stu_survey: &Vec<i32>, men_survey: &Vec<i32>) -> i32 {
        let mut cnt = 0;

        for i in 0 .. stu_survey.len() {
            if stu_survey[i] == men_survey[i] {
                cnt += 1;
            }
        }

        cnt
    }
    fn dp(score: i32, stu_id: usize, students: &Vec<Vec<i32>>, mentors: &Vec<Vec<i32>>, men_visited: &mut Vec<bool>, cache: &mut Vec<Vec<i32>>, max_score: &mut i32) {
        if stu_id == students.len() {
            *max_score = max_score.clone().max(score);
        }


        for i in 0 .. men_visited.len() {
            if men_visited[i] {
                continue;
            }

            if cache[stu_id][i] == -1 { 
                cache[stu_id][i] = Self::get_score(&students[stu_id], &mentors[i]) 
            }
            
            men_visited[i] = true;
            Self::dp(score + cache[stu_id][i], stu_id + 1, students, mentors, men_visited, cache, max_score);
            men_visited[i] = false;
        }
    }
    pub fn max_compatibility_sum(students: Vec<Vec<i32>>, mentors: Vec<Vec<i32>>) -> i32 {
        let mut men_visited = vec![false; students.len()];

        let mut cache: Vec<Vec<i32>> = vec![vec![-1; students.len()]; students.len()];

        let mut max_score: i32 = 0;

        Self::dp(0, 0, &students, &mentors, &mut men_visited, &mut cache, &mut max_score);

        max_score
    }
}