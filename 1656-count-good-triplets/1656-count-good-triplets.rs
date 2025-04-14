impl Solution {
    pub fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
        let mut triplet_count = 0;

        for i in 0 .. arr.len() - 2 {
            for j in i + 1 .. arr.len() - 1 {
                if (arr[i] - arr[j]).abs() > a {
                    continue;
                }

                for k in j + 1 .. arr.len() {
                    if (arr[j] - arr[k]).abs() > b || (arr[i] - arr[k]).abs() > c {
                        continue;
                    }
                    triplet_count += 1;
                }
            }
        }
        triplet_count
    }
}