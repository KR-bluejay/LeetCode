use std::cmp::Ordering;

impl Solution {
    pub fn compare_version(mut version1: String, mut version2: String) -> i32 {
        let version1: Vec<i32> = version1.split('.')
            .map(|v| v.parse::<i32>().unwrap())
            .collect();
        let version2: Vec<i32> = version2.split('.')
            .map(|v| v.parse::<i32>().unwrap())
            .collect();

        match version1[0].cmp(&version2[0]) {
            Ordering::Less => return -1,
            Ordering::Greater => return 1,
            _ => {}
        };

        for i in 1 .. version1.len().max(version2.len()) {
            let version1_tmp = *version1.get(i).unwrap_or(&0);
            let version2_tmp = *version2.get(i).unwrap_or(&0);

            match version1_tmp.cmp(&version2_tmp) {
                Ordering::Less => return -1,
                Ordering::Greater => return 1,
                _ => {},
            }
        }
        0
        
    }
}