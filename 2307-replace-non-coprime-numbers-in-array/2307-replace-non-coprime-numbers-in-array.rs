impl Solution {
    fn gcd(mut left: i32, mut right: i32) -> i32 {
        while right > 0 {
            left %= right;
            std::mem::swap(&mut left, &mut right);
        }

        left
    }
    pub fn replace_non_coprimes(mut nums: Vec<i32>) -> Vec<i32> {
        let mut stack: Vec<i32> = Vec::with_capacity(nums.len());

        for num in nums.iter_mut() {
            while let Some(stack_item) = stack.pop() {
                let gcd = Self::gcd(num.clone(), stack_item.clone());

                if gcd == 1 {
                    stack.push(stack_item);
                    break;
                }
                *num = (*num) / gcd * stack_item;
            }
            stack.push(*num);
        }

        stack
    }
}