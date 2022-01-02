impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        
        // Assumption 1 <= nums.length <= 100
        // The first idea was a dp where
        // dp(x_n) = max(x_n + dp(x_{n-2}, dp(x_{n-1}))
        // Assuming that the solution is solved, the above dp
        // in english meant that the final answer/element is the max i.e choice
        // between the current element or the previous element.
        // Simplifying the above gives me that I only have to store 2 elements to keep track.
        // The 2nd element behind and the 1st element behind the current element 
        if nums.len() == 1{
            return nums[0];
        }
        let mut value1 = nums[0];
        use std::cmp::max;
        if nums.len() == 2 {
            return max(nums[0],nums[1]);
        }
        let mut value2 = max(nums[0], nums[1]);
        
        for i in 2..nums.len() {
            let new_val = max(nums[i]+value1,value2);
            value1 = value2;
            value2 = new_val;
        }
        max(value1,value2)
    }
}
